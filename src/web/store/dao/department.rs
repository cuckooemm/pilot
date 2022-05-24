use super::{master, slaver};

use entity::orm::{DbErr, EntityTrait};
use entity::{DepartmentActive, DepartmentEntity, DepartmentModel};

pub async fn add(de: DepartmentActive) -> Result<u32, DbErr> {
    let r = DepartmentEntity::insert(de).exec(master()).await?;
    Ok(r.last_insert_id)
}

pub async fn get_department() -> Result<Vec<DepartmentModel>, DbErr> {
    Ok(DepartmentEntity::find().all(slaver()).await?)
}
