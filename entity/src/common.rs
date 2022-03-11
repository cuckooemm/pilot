use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "i16", db_type = "Integer")]
pub enum Status {
    #[sea_orm(num_value = 0)]
    Normal,
    #[sea_orm(num_value = 1)]
    Release,
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

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "i16", db_type = "Integer")]
pub enum Premissions {
    #[sea_orm(num_value = 0)]
    Private,
    #[sea_orm(num_value = 1)]
    Public,
}

#[derive(FromQueryResult)]
pub struct ID {
    pub id: i64,
}
