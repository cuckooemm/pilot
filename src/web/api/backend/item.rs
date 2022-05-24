use crate::web::api::permission::accredit;
use crate::web::extract::jwt::Claims;

use super::dao::{item, namespace, publication};
use super::{check, ReqJson, ReqQuery};
use super::{
    response::{APIError, APIResponse, ParamErrType},
    APIResult,
};

use axum::extract::Json;
use entity::orm::Set;
use entity::{utils, ItemActive, ItemCategory, ItemModel, ID};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ItemParam {
    pub id: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub remark: Option<String>,
    pub category: Option<String>,
    pub version: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct PublicationItemParam {
    pub id: Option<String>,
    pub version: Option<u64>,
    pub remark: Option<String>,
}

pub struct PublicationItem {
    pub id: u64,
    pub version: u64,
    pub remark: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PublicationParam {
    pub items: Vec<PublicationItemParam>,
}

#[derive(Deserialize, Serialize)]
pub struct PublicationResult {
    pub successed: Vec<String>,
    pub failed: Vec<String>,
}

pub async fn create(
    ReqJson(param): ReqJson<ItemParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<ID>>> {
    let ns_id = check::id_decode(param.id, "id")?;
    let key = check::id_str(param.key, "key")?;
    let category: ItemCategory = param.category.unwrap_or_default().into();
    let remark = param.remark.unwrap_or_default();
    if remark.len() > 200 {
        return Err(APIError::new_param_err(ParamErrType::Len(1, 200), "remark"));
    }
    // 检查 namespace_id 是否存在
    let info = namespace::get_app_info(ns_id).await?;
    if info.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    let info = info.unwrap();
    // 权限验证 TODO
    if !accredit::accredit(
        &auth,
        entity::rule::Verb::Create,
        vec![&info.app_id, &info.cluster, &info.namespace],
    )
    .await?
    {
        return Err(APIError::new_permission_forbidden());
    }

    // 检查是否已存在此key
    if item::is_key_exist(ns_id, key.clone()).await? {
        return Err(APIError::new_param_err(ParamErrType::Exist, "key"));
    }

    let data = ItemActive {
        namespace_id: Set(ns_id),
        key: Set(key),
        value: Set(param.value.unwrap_or_default()),
        category: Set(category),
        remark: Set(remark),
        version: Set(1u64),
        ..Default::default()
    };

    let id = item::add(data).await?;
    Ok(Json(APIResponse::ok_data(ID::new(id))))
}

pub async fn edit(Json(param): Json<ItemParam>) -> APIResult<Json<APIResponse<ItemModel>>> {
    let item_id = match param.id {
        Some(id) => {
            if id.len() == 0 || id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
            }
            let id = entity::utils::decode_i64(&id);
            if id == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
            }
            id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "id")),
    };
    let version = match param.version {
        Some(version) => {
            if version == 0 {
                return Err(APIError::new_param_err(ParamErrType::Invalid, "version"));
            }
            version
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "version")),
    };
    if let Some(key) = &param.key {
        if key.len() == 0 || key.len() > 100 {
            return Err(APIError::new_param_err(ParamErrType::Len(1, 100), "key"));
        }
    }
    // 校验值类型
    if let Some(remark) = &param.remark {
        if remark.len() > 200 {
            return Err(APIError::new_param_err(ParamErrType::Len(1, 200), "remark"));
        }
    }

    // 找到需要修改的 item
    let entity = item::find_by_id(item_id).await?;
    if entity.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    // let entity = entity.unwrap();
    let success = item::update(
        entity.unwrap(),
        param.key,
        param.value,
        param.category,
        param.remark,
        version,
    )
    .await?;
    if success {
        Ok(Json(APIResponse::ok()))
    } else {
        Err(APIError::new_param_err(ParamErrType::Changed, "item"))
    }
}

#[derive(Deserialize)]
pub struct DetailsParam {
    pub namespace: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

pub async fn list(
    ReqQuery(param): ReqQuery<DetailsParam>,
) -> APIResult<Json<APIResponse<Vec<ItemModel>>>> {
    let ns_id = match param.namespace {
        Some(id) => {
            if id.len() == 0 || id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
            }
            let id = entity::utils::decode_i64(&id);
            if id == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
            }
            id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "id")),
    };
    let (page, page_size) = check::page(param.page, param.page_size);
    let data: Vec<ItemModel> =
        item::find_by_nsid_all(ns_id, (page - 1) * page_size, page_size).await?;
    let mut rsp = APIResponse::ok_data(data);
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}

pub async fn publish(
    ReqJson(param): ReqJson<PublicationParam>,
) -> APIResult<Json<APIResponse<PublicationResult>>> {
    if param.items.len() == 0 {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "items"));
    }
    let mut item_ids = Vec::with_capacity(param.items.len());
    let mut invalid_item_ids = Vec::new();
    for item_param in param.items.into_iter() {
        if let Some(id) = item_param.id {
            if id.len() == 0 || id.len() > 100 {
                invalid_item_ids.push(id);
                continue;
            }
            let item_id = entity::utils::decode_i64(&id);
            let version = item_param.version.unwrap_or_default();
            if item_id == 0 || version == 0 {
                invalid_item_ids.push(id);
                continue;
            }
            item_ids.push(PublicationItem {
                id: item_id,
                version,
                remark: item_param.remark,
            });
        }
    }
    if item_ids.len() == 0 {
        return Err(APIError::new_param_err(ParamErrType::Invalid, "items"));
    }
    let mut rsp = PublicationResult {
        successed: Vec::with_capacity(item_ids.len()),
        failed: Vec::new(),
    };
    for publication in item_ids.iter() {
        // 校验存在 鉴权
        // let data = item::find_by_id(publication.id).await;
        if let Err(err) = publication::publication_item(
            publication.id,
            publication.remark.clone().unwrap_or_default(),
            publication.version,
        )
        .await
        {
            tracing::warn!("failed publish item {}. err: {}", publication.id, err);
            rsp.failed.push(utils::encode_u64(publication.id));
            continue;
        }
        rsp.successed.push(utils::encode_u64(publication.id));
    }
    rsp.failed.append(&mut invalid_item_ids);

    Ok(Json(APIResponse::ok_data(rsp)))
}

#[derive(Deserialize, Serialize)]
pub struct RollbackParam {
    pub record_id: Option<String>,
    pub remark: Option<String>,
}

pub async fn rollback(
    ReqJson(param): ReqJson<RollbackParam>,
) -> APIResult<Json<APIResponse<PublicationResult>>> {
    let record_id = match param.record_id {
        Some(id) => {
            if id.len() == 0 || id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "record_id"));
            }
            let id = entity::utils::decode_i64(&id);
            if id == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "record_id"));
            }
            id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "record_id")),
    };
    let remark = param.remark.unwrap_or("rollback".to_owned());
    publication::rollback_item(record_id, remark).await?;
    Ok(Json(APIResponse::ok()))
}
