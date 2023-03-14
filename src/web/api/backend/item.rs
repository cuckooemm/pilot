use crate::web::api::permission::accredit;
use crate::web::api::{check, helper};
use crate::web::extract::error::{APIError, ForbiddenType, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::Dao;
use crate::web::APIResult;

use axum::extract::State;
use axum::Extension;
use entity::common::enums::Status;
use entity::model::rule::Verb;
use entity::model::{ItemActive, ItemModel, ReleaseHistoryModel, UserAuth};
use entity::orm::{ActiveModelTrait, IntoActiveModel, Set};
use entity::ItemCategory;
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct CreateParam {
    pub namespace_id: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub category: Option<String>,
    pub remark: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn create(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<CreateParam>,
) -> APIResult<APIResponse<ItemModel>> {
    let namespace_id = check::id_decode(param.namespace_id, "namespace_id")?;
    let key = check::key(param.key, "key")?;
    let category = match param.category {
        Some(c) => c
            .try_into()
            .map_err(|_| APIError::param_err(ParamErrType::Invalid, "category"))?,
        None => ItemCategory::Text,
    };
    let remark = param.remark.unwrap_or_default();
    if remark.len() > 200 {
        return Err(APIError::param_err(ParamErrType::Max(200), "remark"));
    }
    let info = dao
        .namespace
        .get_app_info(namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "namespace_id"))?;
    let resource = vec![
        info.app.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, Verb::Create, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Create,
            &resource,
        ));
    }
    if dao.item.is_key_exist(namespace_id, key.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "key"));
    }

    let data = ItemActive {
        namespace_id: Set(namespace_id),
        key: Set(key),
        value: Set(param.value.unwrap_or_default()),
        category: Set(category),
        remark: Set(remark),
        version: Set(1u64),
        ..Default::default()
    };

    let data = dao.item.addition(data).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize, Debug)]
pub struct EditParam {
    pub item_id: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub category: Option<String>,
    pub remark: Option<String>,
    pub status: Option<Status>,
    pub version: Option<u64>,
}

#[instrument(skip(dao, auth))]
pub async fn edit(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<EditParam>,
) -> APIResult<APIResponse<ItemModel>> {
    let item_id = check::id_decode(param.item_id, "item_id")?;
    let version = match param.version {
        Some(version) => {
            if version == 0 {
                return Err(APIError::param_err(ParamErrType::Invalid, "version"));
            }
            version
        }
        None => return Err(APIError::param_err(ParamErrType::Required, "version")),
    };
    if let Some(key) = &param.key {
        check::key_rule(key, "key")?;
    }

    let category: Option<ItemCategory> = param.category.and_then(|c| c.try_into().ok());
    let status: Option<Status> = param.status.and_then(|s| s.try_into().ok());
    if let Some(remark) = &param.remark {
        if remark.len() > 200 {
            return Err(APIError::param_err(ParamErrType::Max(200), "remark"));
        }
    }

    let item = dao
        .item
        .find_by_id(item_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "id"))?;
    if version != item.version {
        return Err(APIError::param_err(ParamErrType::Changed, "version"));
    }
    let info = dao
        .namespace
        .get_app_info(item.namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "id"))?;
    let resource = vec![
        info.app.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, Verb::Modify, &resource).await? {
        return Err(APIError::forbidden_resource(ForbiddenType::Edit, &resource));
    }
    let mut active = item.clone().into_active_model();
    if let Some(key) = param.key {
        if key != item.key {
            active.key = Set(key);
        }
    }
    if let Some(value) = param.value {
        if value != item.value {
            active.value = Set(value);
        }
    }
    if let Some(category) = category {
        if category != item.category {
            active.category = Set(category);
        }
    }
    if let Some(status) = status {
        if status != item.status {
            active.status = Set(status);
        }
    }
    if let Some(remark) = param.remark {
        if remark != item.remark {
            active.remark = Set(remark);
        }
    }
    if !active.is_changed() {
        return Ok(APIResponse::ok_data(item));
    }
    active.version = Set(version + 1);
    let success = dao.item.update(active, version).await?;
    if success {
        let data = dao.item.find_by_id(item_id).await?.unwrap();
        Ok(APIResponse::ok_data(data))
    } else {
        Err(APIError::param_err(ParamErrType::Changed, "item"))
    }
}

#[derive(Deserialize, Debug)]
pub struct DetailsParam {
    pub namespace_id: Option<String>,
    pub status: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn list(
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
    ReqQuery(param): ReqQuery<DetailsParam>,
) -> APIResult<APIResponse<Vec<ItemModel>>> {
    let namespace_id = check::id_decode(param.namespace_id, "namespace_id")?;
    let status: Option<Status> = param.status.and_then(|s| s.try_into().ok());
    let info = dao
        .namespace
        .get_app_info(namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "namespace_id"))?;
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
    let data: Vec<ItemModel> = dao
        .item
        .list_namespace_id(namespace_id, status, helper::page_to_limit(page))
        .await?;
    let mut rsp = APIResponse::ok_data(data);
    rsp.set_page(page);
    Ok(rsp)
}

#[derive(Deserialize, Debug)]
pub struct HistoryListParam {
    pub item_id: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn history_list(
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
    ReqQuery(param): ReqQuery<HistoryListParam>,
) -> APIResult<APIResponse<Vec<ReleaseHistoryModel>>> {
    let item_id = check::id_decode(param.item_id, "item_id")?;
    let namespace_id = dao
        .item
        .get_namespace_id(item_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "item_id"))?;
    let info = dao
        .namespace
        .get_app_info(namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "namespace_id"))?;
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
    let data = dao
        .release
        .list_item(item_id, helper::page_to_limit(page))
        .await?;
    let mut rsp = APIResponse::ok_data(data);
    rsp.set_page(page);
    Ok(rsp)
}
