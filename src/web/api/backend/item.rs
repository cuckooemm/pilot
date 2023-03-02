use crate::web::api::permission::accredit;
use crate::web::api::{check, helper};
use crate::web::extract::error::{APIError, ForbiddenType, ParamErrType};
use crate::web::extract::request::{ReqJson, ReqQuery};
use crate::web::extract::response::APIResponse;
use crate::web::store::dao::Dao;
use crate::web::APIResult;

use axum::extract::State;
use axum::Extension;
use entity::model::rule::Verb;
use entity::model::{ItemActive, ItemCategory, ItemModel, UserAuth};
use entity::orm::Set;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ItemParam {
    pub id: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub category: Option<String>,
    pub remark: Option<String>,
    pub version: Option<i64>,
}

pub async fn create(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<ItemParam>,
) -> APIResult<APIResponse<u64>> {
    let ns_id = check::id_decode(param.id, "id")?;
    let key = check::key(param.key, "key", None, Some(255))?;
    let category: ItemCategory = param.category.unwrap_or_default().into();
    let remark = param.remark.unwrap_or_default();
    if remark.len() > 255 {
        return Err(APIError::param_err(ParamErrType::Len(0, 255), "remark"));
    }
    // 检查 namespace_id 是否存在
    let info = dao.namespace.get_app_info(ns_id).await?;
    if info.is_none() {
        return Err(APIError::param_err(ParamErrType::NotExist, "id"));
    }
    let info = info.unwrap();
    // 权限验证 TODO
    let resource = vec![
        info.app_id.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, Verb::Create, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Operate,
            &resource,
        ));
    }

    // 检查是否已存在此key
    if dao.item.is_key_exist(ns_id, key.clone()).await? {
        return Err(APIError::param_err(ParamErrType::Exist, "key"));
    }

    let data = ItemActive {
        namespace_id: Set(ns_id),
        key: Set(key),
        value: Set(param.value.unwrap_or_default()),
        category: Set(category),
        remark: Set(remark),
        version: Set(1u64),
        ..Default::default()
    };

    let id = dao.item.addition(data).await?;
    Ok(APIResponse::ok_data(id))
}

pub async fn edit(
    Extension(auth): Extension<UserAuth>,
    State(ref dao): State<Dao>,
    ReqJson(param): ReqJson<ItemParam>,
) -> APIResult<APIResponse<ItemModel>> {
    let item_id = check::id_decode(param.id, "id")?;
    let version = match param.version {
        Some(version) => {
            if version == 0 {
                return Err(APIError::param_err(ParamErrType::Invalid, "version"));
            }
            version
        }
        None => return Err(APIError::param_err(ParamErrType::Required, "version")),
    };
    if let Some(key) = &param.key {
        check::key_rule(key, "key", None, Some(255))?;
    }

    let category: ItemCategory = param.category.clone().unwrap_or_default().into();
    // 校验值类型 TODO
    match category {
        ItemCategory::Text => (),
        ItemCategory::Json => (),
        ItemCategory::Yaml => (),
        ItemCategory::Toml => (),
    }
    if let Some(remark) = &param.remark {
        if remark.len() > 255 {
            return Err(APIError::param_err(ParamErrType::Len(0, 255), "remark"));
        }
    }

    // 找到需要修改的 item
    let entity = dao.item.find_by_id(item_id).await?;
    if entity.is_none() {
        return Err(APIError::param_err(ParamErrType::NotExist, "id"));
    }
    let entity = entity.unwrap();
    // 已删除
    // if entity.deleted_at != 0 {
    //     return Err(APIError::param_err(ParamErrType::NotExist, "id"));
    // }
    // 校验权限
    let info = dao.namespace.get_app_info(entity.namespace_id).await?;
    if info.is_none() {
        return Err(APIError::param_err(ParamErrType::NotExist, "id"));
    }
    let info = info.unwrap();
    let resource = vec![
        info.app_id.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, Verb::Modify, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Operate,
            &resource,
        ));
    }

    // let entity = entity.unwrap();
    let success = dao
        .item
        .update(
            entity,
            param.key,
            param.value,
            param.category,
            param.remark,
            version,
            auth.id,
        )
        .await?;
    if success {
        Ok(APIResponse::ok())
    } else {
        Err(APIError::param_err(ParamErrType::Changed, "item"))
    }
}

#[derive(Deserialize)]
pub struct DetailsParam {
    pub namespace_id: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

pub async fn list(
    ReqQuery(param): ReqQuery<DetailsParam>,
    State(ref dao): State<Dao>,
    Extension(auth): Extension<UserAuth>,
) -> APIResult<APIResponse<Vec<ItemModel>>> {
    let ns_id = check::id_decode(param.namespace_id, "namespace_id")?;
    // 校验权限
    let info = dao.namespace.get_app_info(ns_id).await?;
    if info.is_none() {
        return Err(APIError::param_err(ParamErrType::NotExist, "id"));
    }
    let info = info.unwrap();
    let resource = vec![
        info.app_id.as_str(),
        info.cluster.as_str(),
        info.namespace.as_str(),
    ];
    if !accredit::accredit(&auth, Verb::VIEW, &resource).await? {
        return Err(APIError::forbidden_resource(
            ForbiddenType::Operate,
            &resource,
        ));
    }

    let (page, page_size) = helper::page(param.page, param.page_size);
    let data: Vec<ItemModel> = dao
        .item
        .find_by_nsid_all(ns_id, (page - 1) * page_size, page_size)
        .await?;
    let mut rsp = APIResponse::ok_data(data);
    // TODO 更新返回结构 待是否发布标记
    rsp.set_page(page, page_size);
    Ok(rsp)
}
