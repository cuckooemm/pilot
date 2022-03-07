use super::orm::{ConnectionTrait, DatabaseConnection, Schema};

pub async fn create_table(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let stmt = schema.create_table_from_entity(super::ApplicationEntity).if_not_exists().to_owned();
    db.execute(builder.build(&stmt)).await.unwrap();

    let stmt = schema.create_table_from_entity(super::ClusterEntity).if_not_exists().to_owned();
    db.execute(builder.build(&stmt)).await.unwrap();

    let stmt = schema.create_table_from_entity(super::AppNsEntity).if_not_exists().to_owned();
    db.execute(builder.build(&stmt)).await.unwrap();

    let stmt = schema.create_table_from_entity(super::ClusterNsEntity).if_not_exists().to_owned();
    db.execute(builder.build(&stmt)).await.unwrap();

    let stmt = schema.create_table_from_entity(super::ItemEntity).if_not_exists().to_owned();
    db.execute(builder.build(&stmt)).await.unwrap();
}
