use super::check;
use super::orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use super::response::{APIError, APIResponse, ParamErrType};
use super::StoreStats;
use super::{
    APIResult, ClusterColumn, ClusterEntity, NamespaceActive, NamespaceColumn, NamespaceEntity,
    NamespaceModel, ID,
};

use axum::extract::{Extension, Json, Query};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NamespaceParam {
    pub app_id: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<String>,
}

pub async fn create(
    Json(param): Json<NamespaceParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<NamespaceModel>>> {
    let namespace = check::name(param.namespace, "namespace")?;
    let app_id = match param.app_id {
        Some(id) => {
            if id.len() == 0 || id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "app_id")),
    };

    let cluster = match param.cluster {
        Some(cluster) => {
            if cluster.len() == 0 || cluster.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "cluster"));
            }
            cluster
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "cluster")),
    };

    // 查看当前 app_id namespace 是否存在
    let id: Option<ID> = ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Id)
        .filter(ClusterColumn::AppId.eq(app_id.clone()))
        .filter(ClusterColumn::Name.eq(cluster.clone()))
        .into_model::<ID>()
        .one(&store.db)
        .await?;
    if id.is_none() {
        return Err(APIError::new_param_err(
            ParamErrType::NotExist,
            "app_id, cluster",
        ));
    }

    // 查看是否已存在此 namespace
    let id: Option<ID> = NamespaceEntity::find()
        .select_only()
        .column(NamespaceColumn::Id)
        .filter(NamespaceColumn::AppId.eq(app_id.clone()))
        .filter(NamespaceColumn::ClusterName.eq(cluster.clone()))
        .filter(NamespaceColumn::Namespace.eq(namespace.clone()))
        .into_model::<ID>()
        .one(&store.db)
        .await?;
    if id.is_some() {
        return Err(APIError::new_param_err(ParamErrType::Exist, "namespace"));
    }
    let data = NamespaceActive {
        app_id: Set(app_id),
        cluster_name: Set(cluster),
        namespace: Set(namespace),
        ..Default::default()
    };
    let result = data.insert(&store.db).await?;
    Ok(Json(APIResponse::ok(Some(result))))
}

pub async fn list(
    Query(param): Query<NamespaceParam>,
    Extension(store): Extension<StoreStats>,
) -> APIResult<Json<APIResponse<Vec<NamespaceModel>>>> {
    let mut stmt = NamespaceEntity::find();
    if let Some(app_id) = &param.app_id {
        if app_id.len() != 0 {
            if app_id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            stmt = stmt.filter(NamespaceColumn::AppId.eq(app_id.to_string()))
        }
    }
    let list: Vec<NamespaceModel> = stmt.all(&store.db).await?;
    Ok(Json(APIResponse::ok(Some(list))))
}
