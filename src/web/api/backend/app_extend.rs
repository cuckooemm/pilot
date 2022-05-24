use super::dao::app_extend;
use super::response::{APIError, APIResponse, ParamErrType};
use super::{check, APIResult};
use super::{ReqJson, ReqQuery};

use axum::extract::Json;
use entity::orm::Set;
use entity::{AppExtendActive, AppExtendModel};
use entity::{ ID};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppExtendParam {
    pub app_id: Option<String>,
    pub name: Option<String>,
    pub is_public: Option<bool>,
}

pub async fn create(
    ReqJson(param): ReqJson<AppExtendParam>,
) -> APIResult<Json<APIResponse<AppExtendModel>>> {
    let name = check::id_str(param.name, "namespace")?;
    let app_id = check::appid_exist(param.app_id).await?;

    // 查看当前 app_id namespace 是否存在
    let entity: Option<ID> = app_extend::is_exist(&app_id, &name).await?;
    if entity.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "namespace"));
    }

    // let premissions = match param.is_public {
    //     Some(p) => {
    //         if p == true {
    //             Premissions::Public
    //         } else {
    //             Premissions::Private
    //         }
    //     }
    //     None => Premissions::Private,
    // };

    let data = AppExtendActive {
        app_id: Set(app_id),

        ..Default::default()
    };

    let result = app_extend::insert_one(data).await?;
    Ok(Json(APIResponse::ok_data(result)))
}

pub async fn list(
    ReqQuery(param): ReqQuery<AppExtendParam>,
) -> APIResult<Json<APIResponse<Vec<AppExtendModel>>>> {
    // TODO 空字符串没有去除 进入数据库查询会走 where id = ""
    if let Some(app_id) = &param.app_id {
        if app_id.len() != 0 && app_id.len() > 100 {
            return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
        }
    }

    let list: Vec<AppExtendModel> = app_extend::find_by_app_all(param.app_id).await?;
    Ok(Json(APIResponse::ok_data(list)))
}
