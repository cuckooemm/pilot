use super::Conn;

use entity::common::common::Name;
use entity::common::enums::Status;
use entity::model::{DepartmentActive, DepartmentColumn, DepartmentEntity, DepartmentModel};
use entity::orm::sea_query::Expr;
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use entity::ID;

#[derive(Debug, Clone, Default)]
pub struct Department;

impl Department {
    pub async fn addition(&self, name: String) -> Result<DepartmentModel, DbErr> {
        let active = DepartmentActive {
            name: Set(name),
            ..Default::default()
        };
        active.insert(Conn::conn().main()).await
    }

    pub async fn update(&self, active: DepartmentActive) -> Result<DepartmentModel, DbErr> {
        active.update(Conn::conn().main()).await
    }

    pub async fn is_exist_id(&self, id: u32) -> Result<bool, DbErr> {
        let model: Option<u32> = DepartmentEntity::find()
            .select_only()
            .column(DepartmentColumn::Id)
            .filter(DepartmentColumn::Id.eq(id))
            .filter(DepartmentColumn::Status.eq(Status::Normal))
            .into_tuple()
            .one(Conn::conn().main())
            .await?;
        Ok(model.is_some())
    }

    pub async fn get_by_name(&self, name: String) -> Result<Option<DepartmentModel>, DbErr> {
        DepartmentEntity::find()
            .filter(DepartmentColumn::Name.eq(name))
            .one(Conn::conn().main())
            .await
    }

    pub async fn get_info(&self, id: u32) -> Result<Option<DepartmentModel>, DbErr> {
        DepartmentEntity::find_by_id(id)
            .one(Conn::conn().main())
            .await
    }
    pub async fn search_department(
        &self,
        name: Option<String>,
        status: Option<Status>,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<DepartmentModel>, DbErr> {
        let mut stmt = DepartmentEntity::find().offset(offset).limit(limit);
        if let Some(name) = name {
            stmt = stmt.filter(DepartmentColumn::Name.contains(&name));
        }
        if let Some(status) = status {
            stmt = stmt.filter(DepartmentColumn::Status.eq(status));
        }
        Ok(stmt
            .order_by_desc(DepartmentColumn::Id)
            .all(Conn::conn().slaver())
            .await?)
    }
}
