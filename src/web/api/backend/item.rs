use super::orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use super::{check, ReqJson, StoreStats, ID};
use super::{
    response::{APIError, APIResponse, ParamErrType},
    APIResult,
};
use super::{ItemActive, ItemCategory, ItemColumn, ItemEntity, ItemModel};
use super::{NamespaceColumn, NamespaceEntity};

use axum::extract::{Extension, Json, Query};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ItemParam {
    pub id: Option<i64>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub common: Option<String>,
    pub category: Option<String>,
}

pub async fn create(
    ReqJson(param): ReqJson<ItemParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<ItemModel>>> {
    let key = check::name(param.key, "key")?;
    let ns_id = check::number(param.id, "id")?;
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
    let id = NamespaceEntity::find_by_id(ns_id)
        .select_only()
        .column(NamespaceColumn::Id)
        .into_model::<ID>()
        .one(&store.db)
        .await?;
    if id.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "namespace"));
    }
    // 权限验证 TODO

    // 检查是否已存在此key
    let id = ItemEntity::find()
        .select_only()
        .column(ItemColumn::Id)
        .filter(ItemColumn::NsId.eq(ns_id))
        .filter(ItemColumn::Key.eq(key.clone()))
        .into_model::<ID>()
        .one(&store.db)
        .await?;
    if id.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "key"));
    }
    let data = ItemActive {
        ns_id: Set(ns_id),
        key: Set(key),
        value: Set(param.value.unwrap_or_default()),
        category: Set(category),
        comment: Set(common),
        version: Set(0i64),
        ..Default::default()
    };
    let result = data.insert(&store.db).await?;
    Ok(Json(APIResponse::ok(Some(result))))
}

pub async fn update(
    Json(param): Json<ItemParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<ItemModel>>> {
    Err(APIError::new_param_err(ParamErrType::Required, ""))
}

#[derive(Deserialize)]
pub struct DetailsParam {
    pub id: Option<i64>,
}

pub async fn list(
    Query(param): Query<DetailsParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<Vec<ItemModel>>>> {
    let id = check::number(param.id, "id")?;
    let data: Vec<ItemModel> = ItemEntity::find()
        .filter(ItemColumn::NsId.eq(id))
        .all(&store.db)
        .await?;
    Ok(Json(APIResponse::ok(Some(data))))
}
