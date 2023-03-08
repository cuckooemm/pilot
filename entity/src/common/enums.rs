use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

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

impl Default for ItemCategory {
    fn default() -> Self {
        Self::Text
    }
}

impl TryFrom<String> for ItemCategory {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "yaml" => Ok(Self::Yaml),
            "toml" => Ok(Self::Toml),
            "text" => Ok(Self::Text),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "u8", db_type = "TinyUnsigned")]
pub enum Status {
    #[sea_orm(num_value = 0)]
    Normal,
    #[sea_orm(num_value = 2)]
    Band,
    #[sea_orm(num_value = 10)]
    Delete,
}

impl TryFrom<String> for Status {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "delete" => Ok(Self::Delete),
            "normal" => Ok(Self::Normal),
            "band" => Ok(Self::Band),
            _ => Err(()),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Normal
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
            _ => Self::default(),
        }
    }
}
