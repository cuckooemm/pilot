use super::orm::Set;
use super::{check, ReqJson, ReqQuery};
use super::{
    response::{APIError, APIResponse, ParamErrType},
    APIResult,
};
use super::{ItemActive, ItemCategory, ItemModel};

use axum::extract::Json;
use entity::constant::REMARK_MAX_LEN;
use entity::dao::{item, namespace};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ItemParam {
    pub id: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub remark: Option<String>,
    pub category: Option<String>,
}

pub async fn create(ReqJson(param): ReqJson<ItemParam>) -> APIResult<Json<APIResponse<ItemModel>>> {
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
    let remark = param.remark.unwrap_or_default();
    if remark.len() > REMARK_MAX_LEN {
        return Err(APIError::new_param_err(
            ParamErrType::Len(1, REMARK_MAX_LEN),
            "remark",
        ));
    }
    // 检查 namespace_id 是否存在
    let id = namespace::is_exist_by_id(ns_id).await?;
    if id.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
    }
    // 权限验证 TODO

    // 检查是否已存在此key
    let id = item::is_exist_key(ns_id, &key).await?;
    if id.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "key"));
    }
    let data = ItemActive {
        namespace_id: Set(ns_id),
        key: Set(key),
        value: Set(param.value.unwrap_or_default()),
        category: Set(category),
        remark: Set(remark),
        version: Set(0i64),
        ..Default::default()
    };

    let result = item::insert_one(data).await?;
    Ok(Json(APIResponse::ok(Some(result))))
}

pub async fn update(Json(param): Json<ItemParam>) -> APIResult<Json<APIResponse<ItemModel>>> {
    Err(APIError::new_param_err(ParamErrType::Required, ""))
}

#[derive(Deserialize)]
pub struct DetailsParam {
    pub id: Option<String>,
}

pub async fn list(
    ReqQuery(param): ReqQuery<DetailsParam>,
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
