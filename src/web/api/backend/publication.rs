use std::collections::HashMap;

use super::APIResult;
use crate::web::api::permission::accredit;
use crate::web::api::{check, helper};
use crate::web::extract::error::{APIError, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::Dao;

use ahash::RandomState;
use axum::extract::{Json, State};
use axum::Extension;
use entity::item::ItemDesc;
use entity::release::ReleaseItemVersion;
use entity::release_history::HistoryItem;
use entity::{UserAuth, ID};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PublicationItemParam {
    pub id: Option<String>,
    pub version: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct PublicationParam {
    pub items: Vec<PublicationItemParam>,
    pub name: Option<String>, // 发布说明
    pub remark: Option<String>,
}

pub async fn publish(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<PublicationParam>,
) -> APIResult<APIResponse<()>> {
    if param.items.len() == 0 {
        return Err(APIError::param_err(ParamErrType::NotExist, "items"));
    }
    let mut new_items = Vec::with_capacity(param.items.len());
    for item_param in param.items.into_iter() {
        if let Some(id) = item_param.id {
            let id = check::id_decode(Some(id), "items.id")?;
            let version = item_param.version.unwrap_or_default();
            if version == 0 {
                return Err(APIError::param_err(ParamErrType::Invalid, "items.version"));
            }
            new_items.push(ReleaseItemVersion { id, version });
        }
    }
    if new_items.is_empty() {
        return Err(APIError::param_err(ParamErrType::Invalid, "items"));
    }
    let release_name = param.name.unwrap_or("publish".to_owned());
    let remark = param.remark.unwrap_or("publish".to_owned());
    if release_name.len() > 64 {
        return Err(APIError::param_err(ParamErrType::Len(0, 64), "name"));
    }
    if remark.len() > 255 {
        return Err(APIError::param_err(ParamErrType::Len(0, 255), "remark"));
    }
    // 获取 item_id 的 namespace
    let item_ids = new_items.iter().map(|i| i.id).collect();
    let db_items = dao.item.get_item_by_ids(item_ids).await?;
    // 返回数量不一致 包含不存在的 item
    if db_items.len() == 0 || db_items.len() != new_items.len() {
        return Err(APIError::param_err(ParamErrType::NotExist, "items"));
    }
    let mut version_map = HashMap::with_capacity_and_hasher(new_items.len(), RandomState::new());
    for i in new_items.into_iter() {
        version_map.insert(i.id, i.version);
    }

    // 获取 namespace_id
    let namespace_id = db_items.first().unwrap().namespace_id;
    // 校验 namespace,versoin 一致
    let mut items_map = HashMap::with_capacity_and_hasher(db_items.len(), RandomState::new());
    // 校验完 namespace_id 后转为 ItemDesc 结构
    let mut db_items_desc = Vec::with_capacity(db_items.len());
    for ida in db_items.into_iter() {
        if let Some(&v) = version_map.get(&ida.id) {
            // items 不是同一个 namespace 报错
            if ida.namespace_id != namespace_id {
                return Err(APIError::param_err(ParamErrType::Invalid, "items"));
            }
            // 版本不一致 有过更新
            if ida.version != v {
                return Err(APIError::param_err(ParamErrType::Changed, "items"));
            }
            let item = ItemDesc {
                id: ida.id,
                key: ida.key.clone(),
                value: ida.value.clone(),
                category: ida.category.clone(),
                version: ida.version,
            };
            db_items_desc.push(item.clone());
            items_map.insert(ida.id, item);
        } else {
            return Err(APIError::param_err(ParamErrType::NotExist, "items"));
        }
    }

    // 权限校验
    // 检查 namespace_id 是否存在
    let info = dao.namespace.get_app_info(namespace_id).await?;
    if info.is_none() {
        return Err(APIError::param_err(ParamErrType::NotExist, "namespace"));
    }
    let info = info.unwrap();
    // 权限验证 TODO
    let resource = vec![
        info.app_id.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, entity::rule::Verb::Publish, &resource).await? {
        return Err(APIError::forbidden_resource(
            crate::web::extract::error::ForbiddenType::Operate,
            &resource,
        ));
    }
    // 获取最后一次发布的配置及配置ID
    let config = dao.release.get_namespace_config(namespace_id).await?;
    let (release_id, release_config) = match config {
        Some(config) => {
            let config_item: Result<Vec<ItemDesc>, serde_json::Error> =
                serde_json::from_str(&config.configurations);
            if config_item.is_err() {
                tracing::error!(
                    "failed to parse release config data: {}, {:?}",
                    &config.configurations,
                    config_item
                );
                return Err(APIError::param_err(ParamErrType::Invalid, "namespace"));
            }

            let mut config_item = config_item.unwrap();
            for i in config_item.iter_mut() {
                if let Some(d) = items_map.remove(&i.id) {
                    // 如果已发布的 version >= 将要发布的version 则可能已经被发布过
                    // 数据可能不一致
                    if i.version >= d.version {
                        return Err(APIError::param_err(ParamErrType::Changed, "items"));
                    }
                    *i = d;
                }
            }
            // 新增的item
            for (_, i) in items_map.into_iter() {
                config_item.push(i);
            }
            (config.id, config_item)
        }
        None => (0, db_items_desc.clone()),
    };

    // 发布
    if !dao
        .release
        .publication_item(
            release_id,
            release_name,
            namespace_id,
            remark,
            release_config,
            db_items_desc,
            auth.id,
        )
        .await?
    {
        // 发生更新  终止发布
        return Err(APIError::param_err(ParamErrType::Changed, "items"));
    }

    Ok(APIResponse::ok())
}

#[derive(Deserialize, Serialize)]
pub struct RollbackParam {
    pub id: Option<String>,
    pub remark: Option<String>,
}

pub async fn rollback(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<RollbackParam>,
) -> APIResult<APIResponse<ID>> {
    let history_id = check::id_decode(param.id, "id")?;
    let namespace_id = dao.release.get_history_namespace_id(history_id).await?;
    if namespace_id.is_none() {
        return Err(APIError::param_err(ParamErrType::NotExist, "id"));
    }
    // 权限校验
    // 检查 namespace_id 是否存在
    let info = dao.namespace.get_app_info(namespace_id.unwrap()).await?;
    if info.is_none() {
        return Err(APIError::param_err(ParamErrType::NotExist, "namespace"));
    }
    let info = info.unwrap();
    // 权限验证 TODO
    if !accredit::accredit(
        &auth,
        entity::rule::Verb::Publish,
        &vec![&info.app_id, &info.cluster, &info.namespace],
    )
    .await?
    {
        return Err(APIError::forbidden_err(
            crate::web::extract::error::ForbiddenType::Operate,
            "",
        ));
    }

    let remark = param.remark.unwrap_or("rollback".to_owned());
    dao.release.rollback_item(history_id, remark).await?;
    Ok(APIResponse::ok())
}

#[derive(Deserialize)]
pub struct HistoryParam {
    id: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}
// 获取item 发布记录
pub async fn release_list(
    ReqQuery(param): ReqQuery<HistoryParam>,
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<APIResponse<Vec<HistoryItem>>> {
    let namespace_id = check::id_decode(param.id, "id")?;
    // 权限校验
    // 检查 namespace_id 是否存在
    let info = dao.namespace.get_app_info(namespace_id).await?;
    if info.is_none() {
        return Err(APIError::param_err(ParamErrType::NotExist, "namespace"));
    }
    let info = info.unwrap();
    // 权限验证 TODO
    let resource = vec![
        info.app_id.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, entity::rule::Verb::VIEW, &resource).await? {
        return Err(APIError::forbidden_resource(
            crate::web::extract::error::ForbiddenType::Operate,
            &resource,
        ));
    }

    let (page, page_size) = helper::page(param.page, param.page_size);
    let history = dao
        .release
        .get_namespace_history(namespace_id, (page - 1) * page_size, page_size)
        .await?;
    let mut rsp = APIResponse::ok_data(history);
    rsp.set_page(page, page_size);
    Ok(rsp)
}
