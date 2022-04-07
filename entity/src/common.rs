use std::fmt::Display;
use crate::grable_id;

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
#[sea_orm(rs_type = "i16", db_type = "Integer")]
pub enum Premissions {
    #[sea_orm(num_value = 0)]
    Private,
    #[sea_orm(num_value = 1)]
    Public,
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
