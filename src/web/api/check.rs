use crate::web::extract::response::{APIError, ParamErrType};
use crate::web::store::dao::app;

use entity::constant::*;
use entity::orm::ConnectionTrait;
use entity::ID;

// 检查 app_id 参数
pub fn app_id(id: Option<String>) -> Result<String, APIError> {
    match id {
        Some(id) => {
            if id.len() < APP_ID_MIN_LEN || id.len() > APP_ID_MAX_LEN {
                return Err(APIError::new_param_err(
                    ParamErrType::Len(APP_ID_MIN_LEN, APP_ID_MAX_LEN),
                    "app_id",
                ));
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

pub fn page(page: Option<String>, page_size: Option<String>) -> (u64, u64) {
    let page = page.unwrap_or_default();
    let mut page = page.parse::<u64>().unwrap_or(1);
    if page < 1 {
        page = 1;
    }
    let page_size = page_size.unwrap_or_default();
    let mut page_size = page_size.parse::<u64>().unwrap_or(20);
    if page_size < 1 {
        page_size = 20;
    }
    if page_size > 10000 {
        page_size = 10000;
    }
    if u64::MAX / page_size < page {
        page = 1;
    }
    (page, page_size)
}

// 检查 name 参数
pub fn name(name: Option<String>, field: &str) -> Result<String, APIError> {
    match name {
        Some(name) => {
            if name.len() < NAME_MIN_LEN || name.len() > NAME_MAX_LEN {
                return Err(APIError::new_param_err(
                    ParamErrType::Len(NAME_MIN_LEN, NAME_MAX_LEN),
                    field,
                ));
            }
            Ok(name)
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, field)),
    }
}

// 检查 appid 是否存在
pub async fn appid_exist(app_id: Option<String>) -> Result<String, APIError> {
    match app_id {
        Some(id) => {
            if id.len() < APP_ID_MIN_LEN || id.len() > APP_ID_MAX_LEN {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            // 查找 app_id 是否存在
            let entity: Option<ID> = app::is_exist(id.clone()).await?;
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
    id: Option<String>,
    cluster: Option<String>,
) -> Result<String, APIError>
where
    C: ConnectionTrait,
{
    let id = match id {
        Some(id) => {
            if id.len() == 0 || id.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            id
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "app_id")),
    };
    let cluster = match cluster {
        Some(cluster) => {
            if cluster.len() == 0 || cluster.len() > 100 {
                return Err(APIError::new_param_err(ParamErrType::NotExist, "app_id"));
            }
            cluster
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "cluster")),
    };

    Ok("".to_owned())
}
