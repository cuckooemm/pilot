use super::Conn;

use entity::app::DepartmentID;
use entity::enums::Status;
use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult, Iterable, QueryFilter,
    QuerySelect, Set, TransactionError, TransactionTrait,
};
use entity::rule::Verb;
use entity::{
    AppActive, AppColumn, AppEntity, AppModel, IDu32, RoleActive, RoleEntity, RoleRuleActive,
    RoleRuleEntity, RuleActive, RuleEntity, UserRoleActive, UserRoleEntity, ID,
};

#[derive(Debug, Clone, Default)]
pub struct App;

impl App {
    pub async fn addition(&self, active: AppActive, user_id: u32) -> Result<AppModel, DbErr> {
        let app = active.app.as_ref().clone();
        // 构造默认角色
        let role = RoleActive {
            name: Set(format!("{}.{}", "Main", &app)),
            ..Default::default()
        };
        // 构造用户角色
        let mut user_bind = UserRoleActive {
            user_id: Set(user_id),
            ..Default::default()
        };
        // 构造权限
        let mut rules: Vec<RuleActive> = Vec::with_capacity(Verb::iter().len());
        for v in Verb::iter() {
            let rule = RuleActive {
                verb: Set(v),
                resource: Set(app.clone()),
                ..Default::default()
            };
            rules.push(rule);
        }
        let mut binds: Vec<RoleRuleActive> = Vec::with_capacity(rules.len());

        return Conn::conn()
            .main()
            .transaction::<_, AppModel, DbErr>(|tx| {
                Box::pin(async move {
                    // create app
                    let app = active.insert(tx).await?;
                    // create Role
                    let role_id = RoleEntity::insert(role).exec(tx).await?.last_insert_id;
                    // create rule
                    let rule_count = rules.len() as u64;
                    let rule_id = RuleEntity::insert_many(rules)
                        .exec(tx)
                        .await?
                        .last_insert_id;
                    // 角色绑定权限
                    for id in rule_id..(rule_id + rule_count) {
                        let bind = RoleRuleActive {
                            role_id: Set(role_id),
                            rule_id: Set(id),
                            ..Default::default()
                        };
                        binds.push(bind);
                    }
                    RoleRuleEntity::insert_many(binds).exec(tx).await?;
                    // 为用户添加角色
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

    pub async fn find_all(&self, (offset, limit): (u64, u64)) -> Result<Vec<AppModel>, DbErr> {
        AppEntity::find()
            .offset(offset)
            .limit(limit)
            .all(Conn::conn().main())
            .await
    }

    pub async fn get_info(&self, app: String) -> Result<Option<AppModel>, DbErr> {
        AppEntity::find()
            .filter(AppColumn::App.eq(app))
            .one(Conn::conn().main())
            .await
    }
    pub async fn get_app_department_by_id(&self, app: String) -> Result<Option<u32>, DbErr> {
        AppEntity::find()
            .select_only()
            .column(AppColumn::DeptId)
            .filter(AppColumn::App.eq(app))
            .into_model::<DepartmentID>()
            .one(Conn::conn().main())
            .await
            .map(|id| id.and_then(|id| Some(id.dept_id)))
    }
    pub async fn get_app_id(&self, app: String) -> Result<Option<u32>, DbErr> {
        AppEntity::find()
            .select_only()
            .column(AppColumn::Id)
            .filter(AppColumn::App.eq(app))
            .into_model::<IDu32>()
            .one(Conn::conn().main())
            .await
            .map(|id| id.and_then(|id| Some(id.id)))
    }

    // 查找 app_id 是否存在
    pub async fn is_exist(&self, app: String) -> Result<bool, DbErr> {
        AppEntity::find()
            .select_only()
            .column(AppColumn::Id)
            .filter(AppColumn::App.eq(app))
            .into_model::<ID>()
            .one(Conn::conn().main())
            .await
            .and_then(|id| Ok(id.is_some()))
    }
}
