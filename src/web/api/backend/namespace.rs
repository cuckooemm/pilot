use crate::web::api::permission::accredit;
use crate::web::api::{check, helper};
use crate::web::extract::error::{APIError, ForbiddenType, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::Dao;
use crate::web::APIResult;

use axum::extract::State;
use axum::Extension;
use entity::common::enums::Status;
use entity::model::{ rule::Verb, NamespaceActive, NamespaceModel, UserAuth};
use entity::orm::{ActiveModelTrait, IntoActiveModel, Set};
use entity::Scope;
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct CreateParam {
    pub app: Option<String>,
    pub cluster: Option<String>,
    pub namespace: Option<String>,
    pub scope: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn create(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<CreateParam>,
) -> APIResult<APIResponse<NamespaceModel>> {
    let namespace = check::id_str(param.namespace, "namespace")?;
    let app = check::id_str(param.app, "app")?;
    let scope = Scope::from(param.scope.unwrap_or_default());
    let cluster = check::id_str(param.cluster, "cluster")?;
    let resource = vec![app.as_str(), cluster.as_str()];
    if !accredit::accredit(&auth, Verb::Create, &vec![&app, &cluster]).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Create,
            &resource,
        ));
    }
    if !dao.cluster.is_exist(app.clone(), cluster.clone()).await? {
        return Err(APIError::param_err(ParamErrType::NotExist, "app, cluster"));
    }
    if dao
        .namespace
        .is_exist(app.clone(), cluster.clone(), namespace.clone())
        .await?
    {
        return Err(APIError::param_err(ParamErrType::Exist, "namespace"));
    }

    let active = NamespaceActive {
        app: Set(app),
        cluster: Set(cluster),
        namespace: Set(namespace),
        scope: Set(scope),
        ..Default::default()
    };
    let data = dao.namespace.addition(active).await?;
    Ok(APIResponse::ok_data(data))
}

#[derive(Deserialize, Debug)]
pub struct EditParam {
    pub id: Option<String>,
    pub scope: Option<String>,
    pub status: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn edit(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<EditParam>,
) -> APIResult<APIResponse<NamespaceModel>> {
    let namespace_id = check::id_decode(param.id, "id")?;
    let namespace = dao
        .namespace
        .get_namespace_by_id(namespace_id)
        .await?
        .ok_or(APIError::param_err(ParamErrType::NotExist, "id"))?;
    let resource = vec![
        namespace.app.as_str(),
        namespace.cluster.as_str(),
        namespace.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, Verb::Modify, &resource).await? {
        return Err(APIError::forbidden_resource(ForbiddenType::Edit, &resource));
    }
    let mut active = namespace.clone().into_active_model();
    if let Some(status) = param.status.and_then(|s| s.try_into().ok()) {
        if status != namespace.status {
            active.status = Set(status);
        }
    }
    if let Some(scope) = param.scope {
        let scope: Scope = scope.into();
        if scope != Scope::Public {
            return Err(APIError::param_err(ParamErrType::Invalid, "scope"));
        }
        if namespace.scope == Scope::Private {
            active.scope = Set(scope);
        }
    }
    if !active.is_changed() {
        return Ok(APIResponse::ok_data(namespace));
    }
    let rsp = dao.namespace.update(active).await?;
    Ok(APIResponse::ok_data(rsp))
}

#[derive(Deserialize, Debug)]
pub struct QueryParam {
    pub app: Option<String>,
    pub cluster: Option<String>,
    pub status: Option<String>,
    pub scope: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

#[instrument(skip(dao, auth))]
pub async fn list(
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
    ReqQuery(param): ReqQuery<QueryParam>,
) -> APIResult<APIResponse<Vec<NamespaceModel>>> {
    let app = check::id_str(param.app, "app")?;
    let cluster = check::id_str(param.cluster, "cluster")?;
    let status: Option<Status> = param.status.and_then(|s| s.try_into().ok());
    let page = helper::page(param.page, param.page_size);
    let scope = param.scope.and_then(|s| Some(s.into()));
    let resource = vec![app.as_str(), cluster.as_str()];
    if !accredit::accredit(&auth, Verb::VIEW, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Access,
            &resource,
        ));
    }
    let list = dao
        .namespace
        .list_by_appcluster(
            app.clone(),
            cluster.clone(),
            status,
            scope,
            helper::page_to_limit(page),
        )
        .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page);
    Ok(rsp)
}

#[derive(Deserialize, Debug)]
pub struct QueryPublucParam {
    pub namespace: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

#[instrument(skip(dao))]
pub async fn list_public(
    State(ref dao): State<Dao>,
    ReqQuery(param): ReqQuery<QueryPublucParam>,
) -> APIResult<APIResponse<Vec<NamespaceModel>>> {
    let namespace = match param.namespace {
        Some(n) => {
            let name = check::id_str_rule(n, "namespace")?;
            Some(name)
        }
        None => None,
    };
    let page = helper::page(param.page, param.page_size);
    let list = dao
        .namespace
        .get_public_namespace_info(namespace, page)
        .await?;
    let mut rsp = APIResponse::ok_data(list);
    rsp.set_page(page);
    Ok(rsp)
}
