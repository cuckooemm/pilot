use super::Conn;

use entity::model::{
    common::{Id32Name, Name},
    enums::Status,
    DepartmentActive, DepartmentColumn, DepartmentEntity, DepartmentModel, ID,
};
use entity::orm::sea_query::Expr;
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};

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

    // pub async fn delete(&self, name: String) -> Result<u64, DbErr> {
    //     let r = DepartmentEntity::update_many()
    //         .col_expr(
    //             DepartmentColumn::DeletedAt,
    //             Expr::value(Local::now().timestamp() as u64),
    //         )
    //         .filter(DepartmentColumn::Name.eq(name))
    //         .filter(DepartmentColumn::DeletedAt.eq(0_u64))
    //         .exec(Conn::conn().main())
    //         .await?;
    //     Ok(r.rows_affected)
    // }

    pub async fn is_exist_id(&self, id: u32) -> Result<bool, DbErr> {
        let model = DepartmentEntity::find()
            .select_only()
            .column(DepartmentColumn::Id)
            .filter(DepartmentColumn::Id.eq(id))
            .into_model::<ID>()
            .one(Conn::conn().main())
            .await?;
        Ok(model.is_some())
    }

    pub async fn is_exist(&self, name: String) -> Result<bool, DbErr> {
        let model = DepartmentEntity::find()
            .select_only()
            .column(DepartmentColumn::Id)
            .filter(DepartmentColumn::Name.eq(name))
            .into_model::<ID>()
            .one(Conn::conn().main())
            .await?;
        Ok(model.is_some())
    }

    pub async fn get_info(&self, id: u32) -> Result<Option<DepartmentModel>, DbErr> {
        DepartmentEntity::find_by_id(id)
            .one(Conn::conn().main())
            .await
    }
    pub async fn get_department_name(&self, id: u32) -> Result<Option<String>, DbErr> {
        let r = DepartmentEntity::find()
            .select_only()
            .column(DepartmentColumn::Name)
            .filter(DepartmentColumn::Id.eq(id))
            .into_model::<Name>()
            .one(Conn::conn().slaver())
            .await?;
        Ok(r.and_then(|s| Some(s.name)))
    }

    pub async fn search_department(
        &self,
        name: Option<String>,
        status: Option<Status>,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<Id32Name>, DbErr> {
        let mut stmt = DepartmentEntity::find()
            .select_only()
            .column(DepartmentColumn::Id)
            .column(DepartmentColumn::Name)
            .offset(offset)
            .limit(limit);
        if let Some(name) = name {
            stmt = stmt.filter(DepartmentColumn::Name.contains(&name));
        }
        if let Some(status) = status {
            stmt = stmt.filter(DepartmentColumn::Status.eq(status));
        }
        Ok(stmt
            .order_by_desc(DepartmentColumn::Id)
            .into_model::<Id32Name>()
            .all(Conn::conn().slaver())
            .await?)
    }
}
