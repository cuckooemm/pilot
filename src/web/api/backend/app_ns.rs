use super::orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use super::response::{APIError, APIResponse, ParamErrType};
use super::{check, APIResult, AppNsActive, AppNsColumn, AppNsEntity, AppNsModel, Premissions, ID};
use super::{ReqJson, StoreStats};

use axum::extract::{Extension, Json, Query};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppNsParam {
    pub app_id: Option<String>,
    pub namespace: Option<String>,
    pub is_public: Option<bool>,
}

pub async fn create(
    ReqJson(param): ReqJson<AppNsParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<AppNsModel>>> {
    let namespace = check::name(param.namespace, "namespace")?;
    let app_id = check::appid_exist(&store.db, param.app_id).await?;

    // 查看当前 app_id namespace 是否存在
    let entity: Option<ID> = AppNsEntity::find()
        .select_only()
        .column(AppNsColumn::Id)
        .filter(AppNsColumn::AppId.eq(app_id.clone()))
        .filter(AppNsColumn::Namespace.eq(namespace.clone()))
        .into_model::<ID>()
        .one(&store.db)
        .await?;
    if entity.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "namespace"));
    }

    let premissions = match &param.is_public {
        Some(p) => {
            if *p == true {
                Premissions::Public
            } else {
                Premissions::Private
            }
        }
        None => Premissions::Private,
    };

    let data = AppNsActive {
        app_id: Set(app_id),
        namespace: Set(namespace),
        premissions: Set(premissions),
        ..Default::default()
    };
    let result = data.insert(&store.db).await?;
    Ok(Json(APIResponse::ok(Some(result))))
}

pub async fn list(
    Query(param): Query<AppNsParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<Vec<AppNsModel>>>> {
    let mut stmt = AppNsEntity::find();
    if let Some(app_id) = &param.app_id {
        if app_id.len() != 0 {
            if app_id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            stmt = stmt.filter(AppNsColumn::AppId.eq(app_id.to_string()))
        }
    }
    let list: Vec<AppNsModel> = stmt.all(&store.db).await?;
    Ok(Json(APIResponse::ok(Some(list))))
}
