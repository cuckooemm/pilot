use super::Conn;

use entity::common::enums::Status;
use entity::model::{
    rule::Verb, AppActive, AppColumn, AppEntity, AppModel, RoleActive, RoleEntity, RoleRuleActive,
    RoleRuleEntity, RuleActive, RuleEntity, UserRoleActive, UserRoleEntity,
};
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, Iterable, Order, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionError, TransactionTrait,
};

#[derive(Debug, Clone, Default)]
pub struct App;

impl App {
    pub async fn addition(&self, active: AppActive, user_id: u32) -> Result<AppModel, DbErr> {
        let app = active.app.as_ref().clone();
        let role = RoleActive {
            name: Set(format!("{}.{}", "Main", &app)),
            ..Default::default()
        };
        let mut user_bind = UserRoleActive {
            user_id: Set(user_id),
            ..Default::default()
        };
        let mut rules: Vec<RuleActive> = Vec::with_capacity(Verb::iter().len());
        for v in Verb::iter() {
            rules.push(RuleActive {
                verb: Set(v),
                resource: Set(app.clone()),
                ..Default::default()
            });
        }
        let rule_count = rules.len() as u64;
        let mut binds: Vec<RoleRuleActive> = Vec::with_capacity(rules.len());
        return Conn::conn()
            .main()
            .transaction::<_, AppModel, DbErr>(|tx| {
                Box::pin(async move {
                    let app = active.insert(tx).await?;
                    let role_id = RoleEntity::insert(role).exec(tx).await?.last_insert_id;
                    let rule_id = RuleEntity::insert_many(rules)
                        .exec(tx)
                        .await?
                        .last_insert_id;
                    for id in rule_id..(rule_id + rule_count) {
                        binds.push(RoleRuleActive {
                            role_id: Set(role_id),
                            rule_id: Set(id),
                            ..Default::default()
                        });
                    }
                    RoleRuleEntity::insert_many(binds).exec(tx).await?;
                    user_bind.role_id = Set(role_id);
                    UserRoleEntity::insert(user_bind).exec(tx).await?;
                    Ok(app)
                })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Connection(err) => {
                    return err;
                }
                TransactionError::Transaction(err) => {
                    return DbErr::Custom(err.to_string());
                }
            });
    }

    pub async fn update(&self, app: AppActive) -> Result<AppModel, DbErr> {
        app.update(Conn::conn().main()).await
    }
    pub async fn get_apps(
        &self,
        department_id: Option<u32>,
        status: Option<Status>,
        (offset, limit): (u64, u64),
    ) -> Result<Vec<AppModel>, DbErr> {
        let mut stmt = AppEntity::find()
            .order_by(AppColumn::Id, Order::Desc)
            .offset(offset)
            .limit(limit);
        if let Some(department_id) = department_id {
            stmt = stmt.filter(AppColumn::DepartmentId.eq(department_id));
        }
        if let Some(status) = status {
            stmt = stmt.filter(AppColumn::Status.eq(status));
        }
        stmt.all(Conn::conn().slaver()).await
    }

    pub async fn get_info(&self, app: String) -> Result<Option<AppModel>, DbErr> {
        AppEntity::find()
            .filter(AppColumn::App.eq(app))
            .one(Conn::conn().main())
            .await
    }
    pub async fn get_department_id_by_app(&self, app: String) -> Result<Option<u32>, DbErr> {
        AppEntity::find()
            .select_only()
            .column(AppColumn::DepartmentId)
            .filter(AppColumn::App.eq(app))
            .into_tuple()
            .one(Conn::conn().slaver())
            .await
    }

    pub async fn is_exist(&self, app: String) -> Result<bool, DbErr> {
        AppEntity::find()
            .select_only()
            .column(AppColumn::Id)
            .filter(AppColumn::App.eq(app))
            .into_tuple::<u32>()
            .one(Conn::conn().main())
            .await
            .and_then(|id| Ok(id.is_some()))
    }
}
