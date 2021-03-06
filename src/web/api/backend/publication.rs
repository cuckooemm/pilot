use std::collections::HashMap;

use super::dao::{item, release_history};
use super::response::{APIError, ApiResponse, ParamErrType};
use super::APIResult;
use super::{check, ReqJson, ReqQuery};
use crate::web::api::permission::accredit;
use crate::web::extract::jwt::Claims;
use crate::web::store::dao::{namespace, release};

use ahash::RandomState;
use axum::extract::Json;
use entity::item::ItemDesc;
use entity::release::ReleaseItemVersion;
use entity::release_history::HistoryItem;
use entity::ID;
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
    ReqJson(param): ReqJson<PublicationParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<ID>>> {
    if param.items.len() == 0 {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "items"));
    }
    let mut new_items = Vec::with_capacity(param.items.len());
    for item_param in param.items.into_iter() {
        if let Some(id) = item_param.id {
            let id = check::id_decode(Some(id), "items.id")?;
            let version = item_param.version.unwrap_or_default();
            if version == 0 {
                return Err(APIError::new_param_err(
                    ParamErrType::Invalid,
                    "items.version",
                ));
            }
            new_items.push(ReleaseItemVersion { id, version });
        }
    }
    if new_items.is_empty() {
        return Err(APIError::new_param_err(ParamErrType::Invalid, "items"));
    }
    let release_name = param.name.unwrap_or("publish".to_owned());
    let remark = param.remark.unwrap_or("publish".to_owned());
    if release_name.len() > 64 {
        return Err(APIError::new_param_err(ParamErrType::Len(0, 64), "name"));
    }
    if remark.len() > 255 {
        return Err(APIError::new_param_err(ParamErrType::Len(0, 255), "remark"));
    }
    // 获取 item_id 的 namespace
    let item_ids = new_items.iter().map(|i| i.id).collect();
    let db_items = item::get_item_by_ids(item_ids).await?;
    // 返回数量不一致 包含不存在的 item
    if db_items.len() == 0 || db_items.len() != new_items.len() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "items"));
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
                return Err(APIError::new_param_err(ParamErrType::Invalid, "items"));
            }
            // 版本不一致 有过更新
            if ida.version != v {
                return Err(APIError::new_param_err(ParamErrType::Changed, "items"));
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
            return Err(APIError::new_param_err(ParamErrType::NotExist, "items"));
        }
    }

    // 权限校验
    // 检查 namespace_id 是否存在
    let info = namespace::get_app_info(namespace_id).await?;
    if info.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
    }
    let info = info.unwrap();
    // 权限验证 TODO
    if !accredit::accredit(
        &auth,
        entity::rule::Verb::Publish,
        vec![&info.app_id, &info.cluster, &info.namespace],
    )
    .await?
    {
        return Err(APIError::new_permission_forbidden());
    }
    // 获取最后一次发布的配置及配置ID
    let config = release::get_namespace_config(namespace_id).await?;
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
                return Err(APIError::new_param_err(ParamErrType::Invalid, "namespace"));
            }

            let mut config_item = config_item.unwrap();
            for i in config_item.iter_mut() {
                if let Some(d) = items_map.remove(&i.id) {
                    // 如果已发布的 version >= 将要发布的version 则可能已经被发布过
                    // 数据可能不一致
                    if i.version >= d.version {
                        return Err(APIError::new_param_err(ParamErrType::Changed, "items"));
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
    if !release::publication_item(
        release_id,
        release_name,
        namespace_id,
        remark,
        release_config,
        db_items_desc,
        auth.user_id,
    )
    .await?
    {
        // 发生更新  终止发布
        return Err(APIError::new_param_err(ParamErrType::Changed, "items"));
    }

    Ok(Json(ApiResponse::ok()))
}

#[derive(Deserialize, Serialize)]
pub struct RollbackParam {
    pub id: Option<String>,
    pub remark: Option<String>,
}

pub async fn rollback(
    ReqJson(param): ReqJson<RollbackParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<ID>>> {
    let history_id = check::id_decode(param.id, "id")?;
    let namespace_id = release_history::get_namespace_id(history_id).await?;
    if namespace_id.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    // 权限校验
    // 检查 namespace_id 是否存在
    let info = namespace::get_app_info(namespace_id.unwrap()).await?;
    if info.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
    }
    let info = info.unwrap();
    // 权限验证 TODO
    if !accredit::accredit(
        &auth,
        entity::rule::Verb::Publish,
        vec![&info.app_id, &info.cluster, &info.namespace],
    )
    .await?
    {
        return Err(APIError::new_permission_forbidden());
    }

    let remark = param.remark.unwrap_or("rollback".to_owned());
    release::rollback_item(history_id, remark).await?;
    Ok(Json(ApiResponse::ok()))
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
    auth: Claims,
) -> APIResult<Json<ApiResponse<Vec<HistoryItem>>>> {
    let namespace_id = check::id_decode(param.id, "id")?;
    // 权限校验
    // 检查 namespace_id 是否存在
    let info = namespace::get_app_info(namespace_id).await?;
    if info.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
    }
    let info = info.unwrap();
    // 权限验证 TODO
    if !accredit::accredit(
        &auth,
        entity::rule::Verb::VIEW,
        vec![&info.app_id, &info.cluster, &info.namespace],
    )
    .await?
    {
        return Err(APIError::new_permission_forbidden());
    }

    let (page, page_size) = check::page(param.page, param.page_size);
    let history =
        release_history::get_namespace_history(namespace_id, (page - 1) * page_size, page_size)
            .await?;
    let mut rsp = ApiResponse::ok_data(history);
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}
