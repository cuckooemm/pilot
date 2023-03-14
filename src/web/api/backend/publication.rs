use std::collections::HashMap;

use super::APIResult;
use crate::web::api::permission::accredit;
use crate::web::api::{check, helper};
use crate::web::extract::error::{APIError, ForbiddenType, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::Dao;

use ahash::RandomState;
use axum::extract::State;
use axum::Extension;
use entity::common::enums::Status;
use entity::model::release::ItemDesc;
use entity::model::release_history::ReleaseAction;
use entity::model::rule::Verb;
use entity::model::UserAuth;
use entity::model::{ReleaseHistoryActive, ReleaseModel};
use entity::orm::{IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Deserialize, Serialize, Debug)]
pub struct PublicationItemParam {
    pub id: Option<String>,
    pub version: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublicationParam {
    pub items: Vec<PublicationItemParam>,
    pub name: Option<String>,
    pub remark: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn publish(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<PublicationParam>,
) -> APIResult<APIResponse<()>> {
    if param.items.len() == 0 {
        return Err(APIError::param_err(ParamErrType::Required, "items"));
    }
    // uniq
    let mut items = HashMap::with_capacity_and_hasher(param.items.len(), RandomState::new());
    for param in param.items.into_iter() {
        if let Some(id) = param.id {
            let id = check::id_decode::<u64>(Some(id), "items.id")?;
            let version = param.version.unwrap_or_default();
            if version == 0 {
                return Err(APIError::param_err(ParamErrType::Invalid, "items.version"));
            }
            items.insert(id, version);
        }
    }
    if items.is_empty() {
        return Err(APIError::param_err(ParamErrType::Invalid, "items"));
    }
    let release_name = param.name.unwrap_or("publish".to_owned());
    if release_name.len() > 64 {
        return Err(APIError::param_err(ParamErrType::Max(64), "name"));
    }
    let remark = param.remark.unwrap_or("publish".to_owned());
    if remark.len() > 200 {
        return Err(APIError::param_err(ParamErrType::Max(200), "remark"));
    }
    let item_ids: Vec<u64> = items.keys().map(|x| x.clone()).collect();
    let db_items = dao.item.get_item_by_ids(item_ids).await?;
    if db_items.len() == 0 || db_items.len() != items.len() {
        return Err(APIError::param_err(ParamErrType::NotExist, "items"));
    }
    let namespace_id = db_items.first().unwrap().namespace_id;
    let info = dao
        .namespace
        .get_app_info(namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "namespace"))?;
    let resource = vec![
        info.app.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, Verb::Publish, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Operate,
            &resource,
        ));
    }
    let mut items_map = HashMap::with_capacity_and_hasher(db_items.len(), RandomState::new());
    for ida in db_items.into_iter() {
        // not same namespace
        if ida.namespace_id != namespace_id {
            return Err(APIError::param_err(ParamErrType::Invalid, "items"));
        }
        if let Some(&v) = items.get(&ida.id) {
            if ida.version != v {
                return Err(APIError::param_err(ParamErrType::Changed, "items"));
            }
            items_map.insert(ida.id, ida);
        } else {
            return Err(APIError::param_err(ParamErrType::NotExist, "items"));
        }
    }
    let (version, release_items, change) =
        match dao.release.get_namespace_last_release(namespace_id).await? {
            Some((version, configurations)) => {
                let mut release_item: Vec<ItemDesc> = serde_json::from_str(&configurations)
                    .map_err(|e| {
                        tracing::error!(
                            "failed to parse namespace: [{}] release config data: {}, {:?}",
                            namespace_id,
                            &configurations,
                            e
                        );
                        APIError::param_err(ParamErrType::Invalid, "namespace")
                    })?;
                let mut change: Vec<ReleaseHistoryActive> = Vec::with_capacity(items_map.len());
                for r in release_item.iter_mut() {
                    if let Some(d) = items_map.remove(&r.id) {
                        if r.version >= d.version {
                            return Err(APIError::param_err(ParamErrType::Changed, "items"));
                        }
                        let mut active = d.clone().into_active_model();
                        active.action = Set(ReleaseAction::Modify);
                        if d.status != Status::Normal {
                            active.action = Set(ReleaseAction::Remove);
                            r.id = 0; // remove item
                            change.push(active);
                            continue;
                        }
                        change.push(active);
                        r.key = d.key;
                        r.value = d.value;
                        r.category = d.category;
                        r.version = d.version;
                    }
                }
                release_item.retain(|r| r.id != 0);
                for item in items_map.into_values() {
                    change.push(item.clone().into_active_model());
                    release_item.push(item.into());
                }
                (version, release_item, change)
            }
            None => {
                let mut change: Vec<ReleaseHistoryActive> = Vec::with_capacity(items_map.len());
                let release_item: Vec<ItemDesc> = items_map
                    .into_values()
                    .map(|v| {
                        change.push(v.clone().into_active_model());
                        v.into()
                    })
                    .collect();
                (0, release_item, change)
            }
        };

    if !dao
        .release
        .publication(
            namespace_id,
            release_name,
            remark,
            version,
            serde_json::to_string(&release_items).unwrap(),
            change,
        )
        .await?
    {
        // 发生更新  终止发布
        return Err(APIError::param_err(ParamErrType::Changed, "items"));
    }

    Ok(APIResponse::ok())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RollbackParam {
    pub version: Option<String>,
    pub target_version: Option<String>,
    pub remark: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn rollback(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<RollbackParam>,
) -> APIResult<APIResponse<()>> {
    let version = check::id_decode(param.version, "version")?;
    let target_version = check::id_decode(param.target_version, "target_version")?;
    let remark = param.remark.unwrap_or("rollback".to_owned());
    if remark.len() > 200 {
        return Err(APIError::param_err(ParamErrType::Max(200), "remark"));
    }
    let (namespace_id, target_configure) = dao
        .release
        .get_release_by_id(target_version)
        .await?
        .ok_or(APIError::param_err(
            ParamErrType::NotExist,
            "target_version",
        ))?;
    let (last_version, cur_configure) = dao
        .release
        .get_namespace_last_release(namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "namespace"))?;
    if version != last_version {
        return Err(APIError::param_err(ParamErrType::Invalid, "version"));
    }
    let info = dao
        .namespace
        .get_app_info(namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "namespace"))?;
    let resource = vec![
        info.app.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, Verb::Publish, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Operate,
            &resource,
        ));
    }
    let cur_items: Vec<ItemDesc> = serde_json::from_str(&cur_configure).unwrap();
    let target_items: Vec<ItemDesc> = serde_json::from_str(&target_configure).unwrap();
    let mut item_map = HashMap::with_capacity_and_hasher(cur_items.len(), RandomState::new());
    let mut change = Vec::new();
    for item in cur_items.into_iter() {
        item_map.insert(item.id, item);
    }
    for item in target_items.into_iter() {
        match item_map.remove(&item.id) {
            Some(d) => {
                if d.version == item.version {
                    continue;
                }
                let mut active = item.into_active_model();
                active.namespace_id = Set(namespace_id);
                active.action = Set(ReleaseAction::Modify);
                change.push(active);
            }
            None => {
                let mut active = item.into_active_model();
                active.namespace_id = Set(namespace_id);
                change.push(active);
            }
        }
    }
    for item in item_map.into_values() {
        let mut active = item.into_active_model();
        active.namespace_id = Set(namespace_id);
        active.action = Set(ReleaseAction::Remove);
        change.push(active);
    }
    dao.release
        .publication(
            namespace_id,
            "rollback".to_owned(),
            remark,
            version,
            target_configure,
            change,
        )
        .await?;
    Ok(APIResponse::ok())
}

#[derive(Deserialize, Debug)]
pub struct HistoryParam {
    namespace_id: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn list(
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
    ReqQuery(param): ReqQuery<HistoryParam>,
) -> APIResult<APIResponse<Vec<ReleaseModel>>> {
    let namespace_id = check::id_decode(param.namespace_id, "namespace_id")?;
    let info = dao
        .namespace
        .get_app_info(namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "namespace"))?;

    let resource = vec![
        info.app.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, Verb::VIEW, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Access,
            &resource,
        ));
    }
    let page = helper::page(param.page, param.page_size);
    let list = dao
        .release
        .get_namespace_release_list(namespace_id, helper::page_to_limit(page))
        .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page);
    Ok(rsp)
}
