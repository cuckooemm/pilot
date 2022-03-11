use super::response::{APIError, APIResponse, ParamErrType};
use super::APIResult;
use super::{check, ReqJson};

use axum::extract::Json;
use entity::constant::NAME_MAX_LEN;
use entity::{NamespaceActive, NamespaceModel, ID};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ReleasePara {
    pub item_ids: Vec<String>,
    pub operation: Option<String>,
}

pub async fn publish(
    ReqJson(param): ReqJson<ReleasePara>,
) -> APIResult<Json<APIResponse<ReleasePara>>> {
    if param.item_ids.len() == 0 {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "item_ids"));
    }
    if let None = param.operation {
        return Err(APIError::new_param_err(ParamErrType::Required, "operation"));
    }
    let mut item_ids = Vec::with_capacity(param.item_ids.len());
    let mut invalid_item_ids = Vec::new();
    for id in param.item_ids.into_iter() {
        if id.len() == 0 || id.len() > 100 {
            invalid_item_ids.push(id);
            continue;
        }
        let item_id = entity::utils::decode_i64(&id);
        if item_id == 0 {
            invalid_item_ids.push(id);
            continue;
        }
        item_ids.push(item_id);
    }
    if item_ids.len() == 0 {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "item_ids"));
    }
    // 判断操作类型 发布
    match param.operation.unwrap().as_str() {
        "publish" => {
            publish_namespace_items(item_ids).await;
        }
        "rollback" => {}
        _ => return Err(APIError::new_param_err(ParamErrType::Invalid, "operation")),
    }

    Ok(Json(APIResponse::ok(None)))
}

async fn publish_namespace_items(item_ids: Vec<i64>) {
    // 根据需要发布的 key 找到已存在发布的值

    // 保存已发布记录到 record 表中

    // 更新 release 表数据

    // 更新 item 表中数据状态
}

async fn rollback_namespace_item() {}
