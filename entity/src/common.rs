use crate::{grable_id, grable_id_u32};

use std::fmt::Display;

use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "u16", db_type = "Integer")]
pub enum Status {
    #[sea_orm(num_value = 0)]
    Normal,
    #[sea_orm(num_value = 1)]
    Publication,
    #[sea_orm(num_value = 2)]
    Delete,
}

impl Default for Status {
    fn default() -> Self {
        Status::Normal
    }
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum ItemCategory {
    #[sea_orm(string_value = "Text")]
    Text,
    #[sea_orm(string_value = "Json")]
    Json,
    #[sea_orm(string_value = "Yaml")]
    Yaml,
    #[sea_orm(string_value = "Toml")]
    Toml,
}
impl Display for ItemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            &Self::Text => "text",
            &Self::Json => "json",
            &Self::Toml => "toml",
            &Self::Yaml => "yaml",
        };
        write!(f, "{}", s)
    }
}

impl From<String> for ItemCategory {
    fn from(str: String) -> Self {
        match str.to_lowercase().as_str() {
            "json" => Self::Json,
            "yaml" => Self::Yaml,
            "toml" => Self::Toml,
            _ => Self::Text,
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "u8", db_type = "TinyUnsigned")]
pub enum Scope {
    #[sea_orm(num_value = 0)]
    Private,
    #[sea_orm(num_value = 1)]
    Public,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Private
    }
}

impl From<String> for Scope {
    fn from(str: String) -> Self {
        match str.to_lowercase().as_str() {
            "private" => Self::Private,
            "public" => Self::Public,
            _ => Self::Private, // 默认 private
        }
    }
}

#[derive(FromQueryResult, Default, Debug, Clone, Serialize)]
pub struct IDu32 {
    #[serde(serialize_with = "grable_id_u32")]
    pub id: u32,
}

#[derive(FromQueryResult, Default, Debug, Clone, Serialize)]
pub struct ID {
    #[serde(serialize_with = "grable_id")]
    pub id: u64,
}

impl ID {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
