// use super::dao::app_extend;
// use super::{check, APIResult};
// use crate::web::api::permission::accredit;
// use crate::web::extract::error::{APIError, ParamErrType};
// use crate::web::extract::request::{ReqJson, ReqQuery};
// use crate::web::extract::response::APIResponse;
// use crate::web::store::dao::{app, namespace, Dao};

// use axum::extract::{Json, State};
// use axum::Extension;
// use entity::namespace::NamespaceItem;
// use entity::orm::Set;
// use entity::ID;
// use entity::{AppExtendActive, UserAuth};
// use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct AppExtendParam {
//     pub app_id: Option<String>,
//     pub namespace_id: Option<String>,
// }

// pub async fn create(
//     Extension(auth): Extension<UserAuth>,
//     State(ref dao): State<Dao>,
//     ReqJson(param): ReqJson<AppExtendParam>,
// ) -> APIResult<APIResponse<()>> {
//     let app_id = check::id_str(param.app_id, "app_id")?;
//     let namespace_id = check::id_decode(param.namespace_id, "namespace_id")?;
//     if !dao.app.is_exist(app_id.clone()).await? {
//         return Err(APIError::param_err(ParamErrType::NotExist, "app_id"));
//     }
//     // 校验权限 是否拥有 app_id 的创建权限
//     let resource = vec![app_id.as_str()];
//     if !accredit::accredit(&auth, entity::rule::Verb::Create, &resource).await? {
//         return Err(APIError::forbidden_resource(
//             crate::web::extract::error::ForbiddenType::Operate,
//             &resource,
//         ));
//     }

//     // 获取 namespace info
//     let namespace_name = dao.namespace.get_namespace_name(namespace_id).await?;
//     if namespace_name.is_none() {
//         return Err(APIError::param_err(ParamErrType::NotExist, "namespace_id"));
//     }
//     let namespace_name = namespace_name.unwrap();
//     if !dao
//         .app_extend
//         .is_exist(app_id.clone(), namespace_name.clone())
//         .await?
//     {
//         return Err(APIError::param_err(ParamErrType::Exist, "namespace_name"));
//     }
//     let data = AppExtendActive {
//         app: Set(app_id),
//         namespace_id: Set(namespace_id),
//         namespace_name: Set(namespace_name),
//         ..Default::default()
//     };

//     dao.app_extend.addition(data).await?;
//     Ok(APIResponse::ok())
// }

// pub async fn list(
//     State(ref dao): State<Dao>,
//     Extension(auth): Extension<UserAuth>,
//     ReqQuery(param): ReqQuery<AppExtendParam>,
// ) -> APIResult<APIResponse<Vec<NamespaceItem>>> {
//     let app_id = check::id_str(param.app_id, "app_id")?;
//     // 校验权限 是否拥有 app_id 的创建权限
//     let resource = vec![app_id.as_str()];
//     if !accredit::accredit(&auth, entity::rule::Verb::VIEW, &resource).await? {
//         return Err(APIError::forbidden_resource(
//             crate::web::extract::error::ForbiddenType::Operate,
//             &resource,
//         ));
//     }
//     let list: Vec<NamespaceItem> = dao.app_extend.get_app_namespace(app_id).await?;
//     Ok(APIResponse::ok_data(list))
// }
