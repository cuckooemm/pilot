use super::orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Schema};

use once_cell::sync::OnceCell;
use sea_orm::DbErr;

static DB_CLI: OnceCell<DatabaseConnection> = OnceCell::new();

pub fn db_cli() -> &'static DatabaseConnection {
    DB_CLI.get().expect("failed to init database client")
}

// 初始化 orm
pub async fn init_orm(opt: ConnectOptions) -> Result<(), DbErr> {
    let db = Database::connect(opt).await?;
    DB_CLI.set(db).expect("failed to init static database client");
    // 初始化表
    create_table(db_cli()).await?;
    // 初始化 时区 加密 等常量
    Ok(())
}

pub async fn create_table(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let stmt = schema
        .create_table_from_entity(crate::AppEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;

    let stmt = schema
        .create_table_from_entity(crate::ClusterEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;

    let stmt = schema
        .create_table_from_entity(crate::AppExtendEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;

    let stmt = schema
        .create_table_from_entity(crate::NamespaceEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;

    let stmt = schema
        .create_table_from_entity(crate::ItemEntity)
        .if_not_exists()
        .to_owned();
    db.execute(builder.build(&stmt)).await?;
    Ok(())
}
