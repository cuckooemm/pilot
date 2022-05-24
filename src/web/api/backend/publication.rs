use super::dao::{item, publication_record};
use super::response::{APIError, APIResponse, ParamErrType};
use super::APIResult;
use super::{check, ReqJson, ReqQuery};

use axum::extract::Json;
use entity::PublicationRecordModel;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RecordParam {
    item_id: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}
// 获取item 发布记录
pub async fn publication_record(
    ReqQuery(param): ReqQuery<RecordParam>,
) -> APIResult<Json<APIResponse<Vec<PublicationRecordModel>>>> {
    let item_id = match param.item_id {
        Some(id) => {
            if id.len() == 0 || id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "item_id"));
            }
            let id = entity::utils::decode_i64(&id);
            if id == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "item_id"));
            }
            id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "item_id")),
    };
    let (page, page_size) = check::page(param.page, param.page_size);
    let record =
        publication_record::find_by_item(item_id, (page - 1) * page_size, page_size).await?;
    let mut rsp = APIResponse::ok_data(record);
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}
