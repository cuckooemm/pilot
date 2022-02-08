use sea_orm::{ConnectionTrait, DatabaseConnection, Schema};

pub async fn create_table(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let app_stmt = schema.create_table_from_entity(super::Application).if_not_exists().to_owned();
    db.execute(builder.build(&app_stmt)).await.unwrap();
}
