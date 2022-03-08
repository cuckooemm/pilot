use std::result::Result;

use super::orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};
use super::super::extract::response::{APIError, ParamErrType};
use super::{AppColumn, AppEntity, ClusterColumn, ClusterEntity, ID};

// 检查 app_id 参数
pub fn app_id(id: Option<String>) -> Result<String, APIError> {
    match id {
        Some(id) => {
            if id.len() == 0 || id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::Len(1, 100), "app_id"));
            }
            Ok(id)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "app_id")),
    }
}

pub fn number(id: Option<i64>, field: &str) -> Result<i64, APIError> {
    match id {
        Some(id) => {
            if id == 0 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, field));
            }
            Ok(id)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, field)),
    }
}

// 检查 name 参数
pub fn name(name: Option<String>, field: &str) -> Result<String, APIError> {
    match name {
        Some(name) => {
            if name.len() == 0 || name.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::Len(1, 100), field));
            }
            Ok(name)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, field)),
    }
}

// 检查 appid 是否存在
pub async fn appid_exist<'a, C>(db: &C, appid: Option<String>) -> Result<String, APIError>
where
    C: ConnectionTrait,
{
    match appid {
        Some(id) => {
            if id.len() == 0 || id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            // 查找 app_id 是否存在
            let entity: Option<ID> = AppEntity::find()
                .select_only()
                .column(AppColumn::Id)
                .filter(AppColumn::AppId.eq(id.clone()))
                .into_model::<ID>()
                .one(db)
                .await?;
            if entity.is_none() {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            Ok(id)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "app_id")),
    }
}

pub async fn app_cluster_exist<'a, C>(
    db: &C,
    id: String,
    namespace: String,
) -> Result<Option<ID>, DbErr>
where
    C: ConnectionTrait,
{
    ClusterEntity::find()
        .select_only()
        .column(ClusterColumn::Id)
        .filter(ClusterColumn::AppId.eq(id))
        .filter(ClusterColumn::Name.eq(namespace))
        .into_model::<ID>()
        .one(db)
        .await
}
