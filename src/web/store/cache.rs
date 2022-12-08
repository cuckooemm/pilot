use std::{collections::HashMap, sync::Arc, time::Duration};

use super::dao::{release, Dao};

use ahash::RandomState;
use axum::extract::FromRef;
use entity::{item::ConfigItem, orm::DbErr};
use serde::Serialize;
use tokio::{
    sync::{
        broadcast::{self, error::RecvError},
        mpsc, RwLock,
    },
    time::{self, MissedTickBehavior},
};

#[derive(Debug, Clone, Default, Serialize)]
pub struct NamespaceItem {
    #[serde(skip_serializing)]
    namespace_id: u64,
    items: Vec<ConfigItem>,
    version: u64,
}

#[derive(Debug, Clone)]
pub struct CacheItem {
    capacity: usize,
    notifaction: Vec<Arc<RwLock<HashMap<u64, broadcast::Sender<NamespaceItem>, RandomState>>>>,
    list: Vec<Arc<RwLock<HashMap<u64, NamespaceItem, RandomState>>>>,
    reserve: broadcast::Sender<NamespaceItem>,
    namespace_id_sender: mpsc::UnboundedSender<u64>,
}

impl CacheItem {
    #[inline]
    pub fn new() -> Self {
        let (reserve, _) = broadcast::channel::<NamespaceItem>(1024);
        let (namespace_id_sender, namespace_id_receiver) = mpsc::unbounded_channel();
        const CAPACITY: usize = 16;
        const MAP_CAPACITY: usize = 64;
        let mut area = Vec::with_capacity(CAPACITY);
        let mut noti = Vec::with_capacity(CAPACITY);
        for _ in 0..CAPACITY {
            area.push(Arc::new(RwLock::new(HashMap::with_capacity_and_hasher(
                MAP_CAPACITY,
                RandomState::new(),
            ))));
            noti.push(Arc::new(RwLock::new(HashMap::with_capacity_and_hasher(
                MAP_CAPACITY,
                RandomState::new(),
            ))));
        }
        let cache = Self {
            capacity: CAPACITY,
            reserve,
            list: area,
            notifaction: noti,
            namespace_id_sender,
        };
        cache.listen_change(namespace_id_receiver);
        cache
    }
    // 订阅 namespace 更新
    // TODO 多个
    pub async fn subscription(
        &self,
        namespace_id: u64,
        version: Option<u64>,
    ) -> Option<NamespaceItem> {
        let mut added_namespace = false;
        // 从缓存中查找
        let version = version.unwrap_or_default();
        match self.get_item_data(namespace_id).await {
            Some(item) => {
                tracing::debug!("get namespace [{}] item data from cache", namespace_id);
                // 有数据 则判断版本是否相同 不同则返回
                // 版本不同 or 无版本信息, 直接返回
                if version != item.version {
                    return Some(item);
                }
                // 版本一致 向下执行 监听channel
            }
            None => {
                // namespace 不存在 添加到监听列表
                let result = self.add_new_namespace(namespace_id, version.clone()).await;
                if result.is_some() {
                    // 如果已获取到值 则直接返回 如果没有 尝试 namespace receiver 是否存在
                    return result;
                }
                added_namespace = true;
            }
        };

        // 已存在记录且版本一致 等待事件
        // 对比完记录版本到订阅channel 中间，channel可能会触发更新而导致不能及时下发最新配置
        // 如果每次同步都发送事件可能会导致协程频繁唤醒 TODO
        // 暂时实现 仅更新时发送更新事件, 此模式下如果发生此低概率事件 则最长下发时间也不会超过一个请求周期
        let item_receiver = self.get_item_receive(namespace_id).await;
        if item_receiver.is_none() {
            // receive一般不为none 为none 重新添加namespace监听
            if added_namespace {
                return None;
            } else {
                return self.add_new_namespace(namespace_id, version).await;
            }
        }
        let mut item_receiver = item_receiver.unwrap();
        loop {
            let rcv = item_receiver.recv().await;
            // 长度为1 获取到的则为最新的数据
            match rcv {
                Ok(data) => {
                    // 对比版本 不相同则返回
                    if version != data.version {
                        return Some(data);
                    }
                }
                Err(err) => match err {
                    RecvError::Closed => return None,
                    RecvError::Lagged(_) => continue,
                },
            }
        }
    }

    #[inline]
    async fn add_new_namespace(&self, namespace_id: u64, version: u64) -> Option<NamespaceItem> {
        // namespace 不存在 添加到监听列表
        tracing::debug!("send namespace [{}]", namespace_id);
        // 先开启订阅 再发送添加的 namespace
        let mut subscribe = self.reserve.subscribe();

        if let Err(err) = self.namespace_id_sender.send(namespace_id) {
            tracing::error!("failed to send namespace [{}] to sync item task", err.0);
            return None;
        }

        loop {
            let rcv = subscribe.recv().await;
            tracing::debug!("receive global subscribe data: {:?}", &rcv);
            match rcv {
                Ok(data) => {
                    // 接收到的 namespace 不是此次需要的 namespace 重新等待
                    if data.namespace_id != namespace_id {
                        continue;
                    }
                    // 该例子可能出现的场景为 在其他服务实例中获取到了配置数据 但在此实例中此namespace 为首次加载。
                    // 如果直接返回, 则version 字段失去意义
                    // 但因为 当前为全局通道, 此namespace只会进入一次此函数, 通道短期内也只会返回一次此item 所以可能会永远阻塞下去直到超时
                    // 且收到全局通道返回, 则说明当前缓存中存在此namespace 的sender
                    // 尝试监听
                    if data.version == version {
                        // 监听item server
                        // 需跳出函数 防止此全局通道阻塞 需要drop
                        return None;
                    }
                    return Some(data);
                }
                Err(err) => {
                    if err == RecvError::Closed {
                        // 通道已关闭, 返回。 理论永远不会执行到此
                        return None;
                    }
                }
            }
        }
    }

    #[inline]
    async fn get_item_data(&self, namespace_id: u64) -> Option<NamespaceItem> {
        let idx = self.calc_area_index(namespace_id);
        let area = self.list[idx].clone();
        let reader = area.read().await;
        if let Some(item) = reader.get(&namespace_id) {
            return Some(item.clone());
        }
        None
    }

    // 计算 namespace 所在索引
    #[inline]
    fn calc_area_index(&self, namespace_id: u64) -> usize {
        namespace_id as usize % self.capacity
    }

    #[inline]
    async fn get_item_sender(&self, namespace_id: u64) -> Option<broadcast::Sender<NamespaceItem>> {
        let idx = self.calc_area_index(namespace_id);
        let notifaction = self.notifaction[idx].clone();
        let reader = notifaction.read().await;
        if let Some(sender) = reader.get(&namespace_id) {
            return Some(sender.clone());
        }
        None
    }

    #[inline]
    async fn set_item_sender(&self, namespace_id: u64, sender: broadcast::Sender<NamespaceItem>) {
        let idx = self.calc_area_index(namespace_id);
        let area = self.notifaction[idx].clone();
        let mut writer = area.write().await;
        writer.insert(namespace_id, sender);
    }

    #[inline]
    async fn get_item_receive(
        &self,
        namespace_id: u64,
    ) -> Option<broadcast::Receiver<NamespaceItem>> {
        let idx = self.calc_area_index(namespace_id);
        let notifaction = self.notifaction[idx].clone();
        let reader = notifaction.read().await;
        if let Some(sender) = reader.get(&namespace_id) {
            let receive = sender.subscribe();
            return Some(receive);
        }
        None
    }
    // 批量设置 item data
    // async fn batch_set_item_data(&self, item: NamespaceItem) {}

    // 设置 item 数据
    #[inline]
    async fn set_item_data(&self, item: NamespaceItem) {
        let idx = self.calc_area_index(item.namespace_id);
        let area = self.list[idx].clone();
        let mut writer = area.write().await;
        writer.insert(item.namespace_id, item);
    }

    // 更新 item 数据, 版本不一致则更新
    // 返回是否更新结果
    #[inline]
    async fn update_item_data(&self, item: NamespaceItem) -> bool {
        // 更新场景少 所以不通过获取写锁对比值更新
        // 而是先通过读锁 如果不一致再申请写锁更新
        if let Some(val) = self.get_item_data(item.namespace_id).await {
            if val.version == item.version {
                // 无需更新
                return false;
            }
        }
        // 不存在值
        // 版本不一致
        // 需要更新 直接覆盖
        self.set_item_data(item).await;
        true
    }

    fn listen_change(&self, mut namespace_receiver: mpsc::UnboundedReceiver<u64>) {
        let self_add_namespace = self.clone();
        let (listen_sender, mut listen_receiver) = mpsc::unbounded_channel::<u64>();
        tokio::spawn(async move {
            loop {
                // 添加新的 namespace 监听
                // 维护 map
                tokio::select! {
                    id = namespace_receiver.recv() => {
                        let id = id.unwrap_or_default();
                        if id == 0 {
                            tracing::warn!("namespace_receiver receiver zero");
                            continue;
                        }
                        let item = load_database_publication(id).await;
                        if item.is_none() {
                            continue;
                        }
                        let item = item.unwrap();
                        // 添加数据至缓存
                        self_add_namespace.set_item_data(item.clone()).await;

                        // 为新的namespace建立事件通道 长度为1 只保存最新数据
                        let (sender, _) = broadcast::channel::<NamespaceItem>(1);
                        self_add_namespace.set_item_sender(id,sender).await;

                        // 向 receive 发送数据 确保在此之前已经向缓存中添加
                        let _ = self_add_namespace.reserve.send(item);

                        // 通知 将namespace加入监听列表
                        let _ = listen_sender.send(id);
                    },
                }
            }
        });
        let sync_item = self.clone();
        tokio::spawn(async move {
            let mut listen_ids: HashMap<u64, usize> = HashMap::new();
            let mut tick = time::interval(Duration::from_secs(3));
            // TODO 监控
            // 如果执行时间过长而定时到期 则跳过 避免堆积
            tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
            loop {
                tokio::select! {
                    _ = tick.tick() => {
                        let mut handlers = Vec::with_capacity(listen_ids.len());
                        // TODO 限制最大运行数量
                        for (&namespace_id,_) in listen_ids.iter() {
                            handlers.push(tokio::spawn(load_database_publication(namespace_id)));
                        }
                        for handler in handlers {
                            match handler.await {
                                Ok(item) => {
                                    // TODO 批量修改 set
                                    if item.is_none() {
                                        continue;
                                    }
                                    let item = item.unwrap();
                                    // sync_item.set_item_data(item.clone()).await;
                                    // 如果发生更新  则发送事件
                                    if sync_item.update_item_data(item.clone()).await{
                                        tracing::info!("send namesoace {} data to sender",&item.namespace_id);
                                        // 向 item channel 通知更新
                                        let _ = sync_item.get_item_sender(item.namespace_id).await.unwrap().send(item);
                                    }
                                },
                                Err(err) => {
                                    tracing::error!("failed to load database task. err: {}",err);
                                }
                            }
                        }
                    },
                    id = listen_receiver.recv() => {
                        let id = id.unwrap_or_default();
                        if id == 0 {
                            tracing::warn!("listen_receiver receiver zero");
                            continue;
                        }
                        listen_ids.insert(id,0);
                    }
                    // TODO 淘汰不再使用的 namespace
                }
            }
        });
    }
}

// 从数据库中加载数据
pub async fn load_database_publication(namespace_id: u64) -> Option<NamespaceItem> {
    for _ in 0..3 {
        let config = Dao::new().release.get_namespace_config(namespace_id).await;
        match config {
            Ok(config) => match config {
                Some(config) => {
                    let items: Result<Vec<ConfigItem>, serde_json::Error> =
                        serde_json::from_str(&config.configurations);
                    if items.is_err() {
                        tracing::error!("failed to parse config item err: {:?}", items);
                        return None;
                    }
                    return Some(NamespaceItem {
                        namespace_id,
                        version: config.id,
                        items: items.unwrap(),
                    });
                }
                None => return None,
            },
            Err(err) => match err {
                DbErr::Conn(e) => {
                    tracing::error!(
                        "failed to get item by {}, err: {}, retry...",
                        namespace_id,
                        e
                    );
                    continue;
                }
                _ => {
                    tracing::error!("failed to get item by {}, err: {}", namespace_id, err);
                    return None;
                }
            },
        }
    }
    None
}
