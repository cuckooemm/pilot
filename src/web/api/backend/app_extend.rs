use super::dao::app_extend;
use super::response::{APIError, ApiResponse, ParamErrType};
use super::{check, APIResult};
use super::{ReqJson, ReqQuery};
use crate::web::api::permission::accredit;
use crate::web::extract::jwt::Claims;
use crate::web::store::dao::{app, namespace};

use axum::extract::Json;
use entity::namespace::NamespaceItem;
use entity::orm::Set;
use entity::AppExtendActive;
use entity::ID;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppExtendParam {
    pub app_id: Option<String>,
    pub namespace_id: Option<String>,
}

pub async fn create(
    ReqJson(param): ReqJson<AppExtendParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<ID>>> {
    let app_id = check::id_str(param.app_id, "app_id")?;
    let namespace_id = check::id_decode(param.namespace_id, "namespace_id")?;
    if !app::is_exist(app_id.clone()).await? {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
    }
    // 校验权限 是否拥有 app_id 的创建权限
    if !accredit::accredit(&auth, entity::rule::Verb::Create, vec![&app_id]).await? {
        return Err(APIError::new_permission_forbidden());
    }

    // 获取 namespace info
    let namespace_name = namespace::get_namespace_name(namespace_id).await?;
    if namespace_name.is_none() {
        return Err(APIError::new_param_err(
            ParamErrType::NotExist,
            "namespace_id",
        ));
    }
    let namespace_name = namespace_name.unwrap();
    if !app_extend::is_exist(app_id.clone(), namespace_name.clone()).await? {
        return Err(APIError::new_param_err(
            ParamErrType::Exist,
            "namespace_name",
        ));
    }
    let data = AppExtendActive {
        app_id: Set(app_id),
        namespace_id: Set(namespace_id),
        namespace_name: Set(namespace_name),
        creator_user: Set(auth.user_id),
        ..Default::default()
    };

    app_extend::add(data).await?;
    Ok(Json(ApiResponse::ok()))
}

pub async fn list(
    ReqQuery(param): ReqQuery<AppExtendParam>,
    auth: Claims,
) -> APIResult<Json<ApiResponse<Vec<NamespaceItem>>>> {
    let app_id = check::id_str(param.app_id, "app_id")?;
    // 校验权限 是否拥有 app_id 的创建权限
    if !accredit::accredit(&auth, entity::rule::Verb::VIEW, vec![&app_id]).await? {
        return Err(APIError::new_permission_forbidden());
    }
    let list: Vec<NamespaceItem> = app_extend::get_app_namespace(app_id).await?;
    Ok(Json(ApiResponse::ok_data(list)))
}
