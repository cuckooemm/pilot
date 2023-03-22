#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "operate_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(serialize_with = "crate::confuse")]
    pub id: u64,
    pub user_id: u32,
    pub department_id: u32,
    pub action: String,
    pub resource: String,
    pub exter: String,
    pub created_at: DateTimeWithTimeZone,
}

