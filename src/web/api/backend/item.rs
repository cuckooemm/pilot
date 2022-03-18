use super::dao::{item, namespace, publication};
use super::{check, ReqJson, ReqQuery};
use super::{
    response::{APIError, APIResponse, ParamErrType},
    APIResult,
};

use axum::extract::Json;
use entity::common::Status;
use entity::constant::{KEY_MAX_LEN, REMARK_MAX_LEN};
use entity::orm::{ActiveModelTrait, Set};
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
    pub version: Option<i64>,
    pub remark: Option<String>,
}

pub struct PublicationItem {
    pub id: i64,
    pub version: i64,
    pub remark: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PublicationParam {
    pub items: Vec<PublicationItemParam>,
    pub operation: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PublicationResult {
    pub successed: Vec<String>,
    pub failed: Vec<String>,
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
    let category: ItemCategory = param.category.unwrap_or_default().into();
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
        version: Set(1i64),
        ..Default::default()
    };

    let result = item::insert_one(data).await?;
    Ok(Json(APIResponse::ok_data(result)))
}

pub async fn edit(Json(param): Json<ItemParam>) -> APIResult<Json<APIResponse<ItemModel>>> {
    let id = match param.id {
        Some(id) => {
            if id.len() == 0 || id.len() > KEY_MAX_LEN {
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
        if key.len() == 0 || key.len() > KEY_MAX_LEN {
            return Err(APIError::new_param_err(
                ParamErrType::Len(1, KEY_MAX_LEN),
                "key",
            ));
        }
    }
    // 校验值类型
    if let Some(remark) = &param.remark {
        if remark.len() > REMARK_MAX_LEN {
            return Err(APIError::new_param_err(
                ParamErrType::Len(1, REMARK_MAX_LEN),
                "remark",
            ));
        }
    }

    // 找到需要修改的 item
    let entity = item::find_by_id(id).await?;
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
    let data: Vec<ItemModel> = item::find_by_nsid_all(ns_id).await?;
    Ok(Json(APIResponse::ok_data(data)))
}

pub async fn publish(
    ReqJson(param): ReqJson<PublicationParam>,
) -> APIResult<Json<APIResponse<PublicationResult>>> {
    if param.items.len() == 0 {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "items"));
    }
    if let None = param.operation {
        return Err(APIError::new_param_err(ParamErrType::Required, "operation"));
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
                version: version,
                remark: item_param.remark,
            });
        }
    }
    if item_ids.len() == 0 {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "items"));
    }
    let mut rsp = PublicationResult {
        successed: Vec::default(),
        failed: Vec::default(),
    };
    // 判断操作类型 发布
    match param.operation.unwrap().as_str() {
        "publish" => {
            let (successed, failed) = publish_namespace_items(item_ids).await;
            rsp.successed = Vec::with_capacity(successed.len());
            for id in successed.iter() {
                rsp.successed.push(utils::encode_i64(id));
            }
            rsp.failed = Vec::with_capacity(failed.len() + invalid_item_ids.len());
            for id in failed.iter() {
                rsp.failed.push(utils::encode_i64(id));
            }
        }
        "rollback" => {
            let (successed, failed) = rollback_namespace_items(item_ids).await;
        }
        _ => return Err(APIError::new_param_err(ParamErrType::Invalid, "operation")),
    }
    rsp.failed.append(&mut invalid_item_ids);

    Ok(Json(APIResponse::ok_data(rsp)))
}

async fn publish_namespace_items(item_ids: Vec<PublicationItem>) -> (Vec<i64>, Vec<i64>) {
    let mut success = Vec::with_capacity(item_ids.len());
    let mut failed = Vec::new();
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
            failed.push(publication.id);
            continue;
        }
        success.push(publication.id);
    }
    (success, failed)
}

async fn rollback_namespace_items(item_ids: Vec<PublicationItem>) -> (Vec<i64>, Vec<i64>) {
    let mut success = Vec::with_capacity(item_ids.len());
    let mut failed = Vec::new();
    for rollback in item_ids.iter() {
        if let Err(err) = publication::rollback_item(rollback.id, rollback.version).await {
            tracing::warn!("failed rollback item {}. err: {}", rollback.id, err);
            failed.push(rollback.id);
            continue;
        }
        success.push(rollback.id);
    }
    (success, failed)
}
