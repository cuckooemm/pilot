use entity::orm::DbErr;
use entity::orm::{ConnectionTrait, DatabaseConnection, Schema};

pub async fn init_table(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let stmt = schema
        .create_table_from_entity(entity::AppEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;

    let stmt = schema
        .create_table_from_entity(entity::ClusterEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;

    let stmt = schema
        .create_table_from_entity(entity::AppExtendEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;

    let stmt = schema
        .create_table_from_entity(entity::NamespaceEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;

    let stmt = schema
        .create_table_from_entity(entity::ItemEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;

    let stmt = schema
        .create_table_from_entity(entity::PublicationEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;
    let stmt = schema
        .create_table_from_entity(entity::PublicationRecordEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;
    Ok(())
}
