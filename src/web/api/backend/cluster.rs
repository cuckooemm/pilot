use super::orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use super::response::{APIError, APIResponse, ParamErrType};
use super::{check, ReqJson, StoreStats};
use super::{APIResult, ClusterActive, ClusterColumn, ClusterEntity, ClusterModel, ID};

use axum::extract::{Extension, Json, Query};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ClusterParam {
    pub app_id: Option<String>,
    pub name: Option<String>,
}

// 创建app集群
pub async fn create(
    ReqJson(param): ReqJson<ClusterParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<ClusterModel>>> {
    // check param
    let cluster_name = check::name(param.name, "name")?;
    let app_id = check::appid_exist(&store.db, param.app_id).await?;

    // 查看当前 app_id cluster_name是否存在
    let entity: Option<ID> = ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Id)
        .filter(ClusterColumn::AppId.eq(app_id.clone()))
        .filter(ClusterColumn::Name.eq(cluster_name.clone()))
        .into_model::<ID>()
        .one(&store.db)
        .await?;
    if entity.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "cluster_name"));
    }

    let data = ClusterActive {
        app_id: Set(app_id),
        name: Set(cluster_name),
        ..Default::default()
    };
    let result = data.insert(&store.db).await?;
    Ok(Json(APIResponse::ok(Some(result))))
}

pub async fn list(
    Query(param): Query<ClusterParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<Vec<ClusterModel>>>> {
    let mut stmt = ClusterEntity::find();
    if let Some(app_id) = &param.app_id {
        if app_id.len() != 0 {
            if app_id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            stmt = stmt.filter(ClusterColumn::AppId.eq(app_id.to_string()))
        }
    }
    let list: Vec<ClusterModel> = stmt.all(&store.db).await?;
    Ok(Json(APIResponse::ok(Some(list))))
}
