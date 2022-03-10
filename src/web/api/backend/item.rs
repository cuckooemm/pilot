use super::orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use super::{check, ReqJson, ID};
use super::{
    response::{APIError, APIResponse, ParamErrType},
    APIResult,
};
use super::{ItemActive, ItemCategory, ItemColumn, ItemEntity, ItemModel};
use super::{NamespaceColumn, NamespaceEntity};

use axum::extract::{Extension, Json, Query};
use entity::dao::{namespace, item};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ItemParam {
    pub id: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub common: Option<String>,
    pub category: Option<String>,
}

pub async fn create(
    ReqJson(param): ReqJson<ItemParam>
) -> APIResult<Json<APIResponse<ItemModel>>> {
    let key = check::name(param.key, "key")?;
    let ns_id = match param.id {
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
    let category = match param.category.unwrap_or_default().as_str() {
        "json" => ItemCategory::Json,
        "yaml" => ItemCategory::Yaml,
        "toml" => ItemCategory::Toml,
        _ => ItemCategory::Text,
    };
    let common = param.common.unwrap_or_default();
    if common.len() > 200 {
        return Err(APIError::new_param_err(ParamErrType::Len(1, 200), "common"));
    }
    // 检查 namespace_id 是否存在
    let id = namespace::is_exist_by_id(ns_id).await?;
    if id.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
    }
    // 权限验证 TODO

    // 检查是否已存在此key
    let id = item::is_exist_key(ns_id,&key).await?;
    if id.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "key"));
    }
    let data = ItemActive {
        namespace_id: Set(ns_id),
        key: Set(key),
        value: Set(param.value.unwrap_or_default()),
        category: Set(category),
        comment: Set(common),
        version: Set(0i64),
        ..Default::default()
    };
    
    let result = item::insert_one(data).await?;
    Ok(Json(APIResponse::ok(Some(result))))
}

pub async fn update(
    Json(param): Json<ItemParam>
) -> APIResult<Json<APIResponse<ItemModel>>> {
    Err(APIError::new_param_err(ParamErrType::Required, ""))
}

#[derive(Deserialize)]
pub struct DetailsParam {
    pub id: Option<String>,
}

pub async fn list(
    Query(param): Query<DetailsParam>
) -> APIResult<Json<APIResponse<Vec<ItemModel>>>> {
    let ns_id = match param.id {
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
    let data: Vec<ItemModel> = item::find_by_id_all(ns_id).await?;
    Ok(Json(APIResponse::ok(Some(data))))
}
