use std::{collections::HashMap};

use axum::{extract::{Path, Query}};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct CfgParam {
    pub app_id: Option<String>,
    pub namespace: Option<String>,
    pub secret: Option<String>,
}

pub async fn get_config(Query(param): Query<CfgParam>) -> String {
    tracing::info!("receive param {:?}",&param);
    format!("receive param {:?} {:?} {:?}",param.app_id,param.namespace,param.secret)
}

