use sea_orm::FromQueryResult;
use serde::Serialize;

use crate::{common::enums::Status, Scope};

#[derive(FromQueryResult, Serialize, Debug)]
pub struct ClusterItem {
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    pub cluster: String,
    pub describe: String,
    pub status: Status,
}

#[derive(FromQueryResult, Serialize, Debug)]
pub struct NamespaceItem {
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    pub namespace: String,
    pub describe: String,
    pub status: Status,
    pub scropt: Scope,
}
