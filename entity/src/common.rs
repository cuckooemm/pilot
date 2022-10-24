use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(FromQueryResult, Default, Debug, Clone, Serialize)]
pub struct Name {
    pub name: String,
}

#[derive(FromQueryResult, Default, Debug, Clone, Serialize)]
pub struct Id32Name {
    #[serde(serialize_with = "super::confuse")]
    pub id: u32,
    pub name: String,
}

#[derive(FromQueryResult, Default, Debug, Clone, Serialize)]
pub struct Id64Name {
    #[serde(serialize_with = "super::confuse")]
    pub id: u64,
    pub name: String,
}

#[derive(FromQueryResult, Default, Debug, Clone, Serialize)]
pub struct ID {
    #[serde(serialize_with = "super::confuse")]
    pub id: u64,
}

#[derive(FromQueryResult, Default, Debug, Clone, Serialize)]
pub struct IDu32 {
    #[serde(serialize_with = "super::confuse")]
    pub id: u32,
}

impl ID {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
