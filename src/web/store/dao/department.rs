use super::{master, slaver};

use chrono::Local;
use entity::common::{Id32Name, Name};
use entity::orm::sea_query::Expr;
use entity::orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect, Set};
use entity::{DepartmentActive, DepartmentColumn, DepartmentEntity, ID};

pub async fn add(name: String) -> Result<u32, DbErr> {
    let active = DepartmentActive {
        name: Set(name),
        ..Default::default()
    };
    let r = DepartmentEntity::insert(active).exec(master()).await?;
    Ok(r.last_insert_id)
}

pub async fn delete(name: String) -> Result<u64, DbErr> {
    let r = DepartmentEntity::update_many()
        .col_expr(
            DepartmentColumn::DeletedAt,
            Expr::value(Local::now().timestamp() as u64),
        )
        .filter(DepartmentColumn::Name.eq(name))
        .filter(DepartmentColumn::DeletedAt.eq(0_u64))
        .exec(master())
        .await?;
    Ok(r.rows_affected)
}

pub async fn is_exist(name: String) -> Result<bool, DbErr> {
    let model = DepartmentEntity::find()
        .select_only()
        .column(DepartmentColumn::Id)
        .filter(DepartmentColumn::Name.eq(name))
        .filter(DepartmentColumn::DeletedAt.eq(0_u64))
        .into_model::<ID>()
        .one(master())
        .await?;
    Ok(model.is_some())
}

pub async fn get_department_name(id: u32) -> Result<Option<String>, DbErr> {
    let r = DepartmentEntity::find()
        .select_only()
        .column(DepartmentColumn::Name)
        .filter(DepartmentColumn::Id.eq(id))
        .filter(DepartmentColumn::DeletedAt.eq(0_u64))
        .into_model::<Name>()
        .one(slaver())
        .await?;
    Ok(r.and_then(|s| Some(s.name)))
}

pub async fn search_department(name: Option<String>) -> Result<Vec<Id32Name>, DbErr> {
    let mut stmt = DepartmentEntity::find()
        .select_only()
        .column(DepartmentColumn::Id)
        .column(DepartmentColumn::Name);
    if let Some(n) = name {
        stmt = stmt.filter(DepartmentColumn::Name.contains(&n));
    }
    let r = stmt
        .filter(DepartmentColumn::DeletedAt.eq(0_u64))
        .into_model::<Id32Name>()
        .all(slaver())
        .await?;
    Ok(r)
}
