use super::dao::{item, namespace};
use super::{check, ReqJson, ReqQuery};
use super::{
    response::{APIError, APIResponse, ParamErrType},
    APIResult,
};
use crate::web::api::permission::accredit;
use crate::web::extract::jwt::Claims;

use axum::extract::Json;
use entity::orm::Set;
use entity::{ItemActive, ItemCategory, ItemModel, ID};
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
    ReqJson(param): ReqJson<ItemParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<ID>>> {
    let ns_id = check::id_decode(param.id, "id")?;
    let key = check::id_str_len(param.key, "key", None, Some(255))?;
    let category: ItemCategory = param.category.unwrap_or_default().into();
    let remark = param.remark.unwrap_or_default();
    if remark.len() > 255 {
        return Err(APIError::new_param_err(ParamErrType::Len(0, 255), "remark"));
    }
    // 检查 namespace_id 是否存在
    let info = namespace::get_app_info(ns_id).await?;
    if info.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    let info = info.unwrap();
    // 权限验证 TODO
    if !accredit::accredit(
        &auth,
        entity::rule::Verb::Create,
        vec![&info.app_id, &info.cluster, &info.namespace],
    )
    .await?
    {
        return Err(APIError::new_permission_forbidden());
    }

    // 检查是否已存在此key
    if item::is_key_exist(ns_id, key.clone()).await? {
        return Err(APIError::new_param_err(ParamErrType::Exist, "key"));
    }

    let data = ItemActive {
        namespace_id: Set(ns_id),
        key: Set(key),
        value: Set(param.value.unwrap_or_default()),
        category: Set(category),
        remark: Set(remark),
        version: Set(1u64),
        modify_user_id: Set(auth.user_id),
        ..Default::default()
    };

    let id = item::add(data).await?;
    Ok(Json(APIResponse::ok_data(ID::new(id))))
}

pub async fn edit(
    Json(param): Json<ItemParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<ItemModel>>> {
    let item_id = check::id_decode(param.id, "id")?;
    let version = match param.version {
        Some(version) => {
            if version == 0 {
                return Err(APIError::new_param_err(ParamErrType::Invalid, "version"));
            }
            version
        }
        None => return Err(APIError::new_param_err(ParamErrType::Required, "version")),
    };
    if let Some(key) = &param.key {
        check::id_str_len_rule(key, "key", None, Some(255))?;
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
            return Err(APIError::new_param_err(ParamErrType::Len(0, 255), "remark"));
        }
    }

    // 找到需要修改的 item
    let entity = item::find_by_id(item_id).await?;
    if entity.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    let entity = entity.unwrap();
    // 已删除
    if entity.deleted_at != 0 {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    // 校验权限
    let info = namespace::get_app_info(entity.namespace_id).await?;
    if info.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    let info = info.unwrap();
    if !accredit::accredit(
        &auth,
        entity::rule::Verb::Modify,
        vec![&info.app_id, &info.cluster, &info.namespace],
    )
    .await?
    {
        return Err(APIError::new_permission_forbidden());
    }

    // let entity = entity.unwrap();
    let success = item::update(
        entity,
        param.key,
        param.value,
        param.category,
        param.remark,
        version,
        auth.user_id,
    )
    .await?;
    if success {
        Ok(Json(APIResponse::ok()))
    } else {
        Err(APIError::new_param_err(ParamErrType::Changed, "item"))
    }
}

#[derive(Deserialize)]
pub struct DetailsParam {
    pub namespace: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
}

pub async fn list(
    ReqQuery(param): ReqQuery<DetailsParam>,
    auth: Claims,
) -> APIResult<Json<APIResponse<Vec<ItemModel>>>> {
    let ns_id = check::id_decode(param.namespace, "id")?;
    // 校验权限
    let info = namespace::get_app_info(ns_id).await?;
    if info.is_none() {
        return Err(APIError::new_param_err(ParamErrType::NotExist, "id"));
    }
    let info = info.unwrap();
    if !accredit::accredit(
        &auth,
        entity::rule::Verb::VIEW,
        vec![&info.app_id, &info.cluster, &info.namespace],
    )
    .await?
    {
        return Err(APIError::new_permission_forbidden());
    }

    let (page, page_size) = check::page(param.page, param.page_size);
    let data: Vec<ItemModel> =
        item::find_by_nsid_all(ns_id, (page - 1) * page_size, page_size).await?;
    let mut rsp = APIResponse::ok_data(data);
    // TODO 更新返回结构 待是否发布标记
    rsp.set_page(page, page_size);
    Ok(Json(rsp))
}
