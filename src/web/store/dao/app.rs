use std::str::FromStr;

use super::master;

use entity::orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, Iterable, QueryFilter, QuerySelect, Set,
    TransactionError, TransactionTrait,
};
use entity::rule::Verb;
use entity::{
    AppActive, AppColumn, AppEntity, AppModel, IDu32, RoleActive, RoleEntity, RoleRuleActive,
    RoleRuleEntity, RuleActive, RuleEntity, UserAuth, UserRoleActive, UserRoleEntity, ID,
};

pub async fn add(app_id: String, name: String, dept_id: u32, auth: &UserAuth) -> Result<(), DbErr> {
    // 构造应用数据
    let app = AppActive {
        app_id: Set(app_id.clone()),
        name: Set(name),
        dept_id: Set(dept_id),
        creator_user: Set(auth.id),
        ..Default::default()
    };
    // 构造默认角色
    let role = RoleActive {
        name: Set(String::from_str("Main").unwrap() + &app_id),
        ..Default::default()
    };
    // 构造用户角色
    let mut user_bind = UserRoleActive {
        user_id: Set(auth.id),
        ..Default::default()
    };
    // 构造权限
    let mut rules: Vec<RuleActive> = Vec::with_capacity(Verb::iter().len());
    for v in Verb::iter() {
        let rule = RuleActive {
            verb: Set(v),
            resource: Set(app_id.clone()),
            ..Default::default()
        };
        rules.push(rule);
    }
    let mut binds: Vec<RoleRuleActive> = Vec::with_capacity(rules.len());

    return master()
        .transaction::<_, (), DbErr>(|tx| {
            Box::pin(async move {
                // 创建应用
                let _ = AppEntity::insert(app).exec(tx).await?;
                // 创建角色
                let role_id = RoleEntity::insert(role).exec(tx).await?.last_insert_id;
                // 创建权限
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
                Ok(())
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(err) => {
                return err;
            }
            TransactionError::Transaction(err) => {
                return DbErr::Exec(err.to_string());
            }
        });
}

pub async fn update(app: AppActive) -> Result<AppModel, DbErr> {
    app.update(master()).await
}

pub async fn find_all(offset: u64, limit: u64) -> Result<Vec<AppModel>, DbErr> {
    AppEntity::find()
        .offset(offset)
        .limit(limit)
        .all(master())
        .await
}

pub async fn get_info(app_id: String) -> Result<Option<AppModel>, DbErr> {
    AppEntity::find()
        .filter(AppColumn::AppId.eq(app_id))
        .one(master())
        .await
}

pub async fn get_app_id(app_id: String) -> Result<Option<u32>, DbErr> {
    let id = AppEntity::find()
        .select_only()
        .column(AppColumn::Id)
        .filter(AppColumn::AppId.eq(app_id))
        .filter(AppColumn::DeletedAt.eq(0_u64))
        .into_model::<IDu32>()
        .one(master())
        .await?;
    Ok(id.and_then(|x| Some(x.id)))
}

// 查找 app_id 是否存在
pub async fn is_exist(app_id: String) -> Result<bool, DbErr> {
    let entiy = AppEntity::find()
        .select_only()
        .column(AppColumn::Id)
        .filter(AppColumn::AppId.eq(app_id))
        .filter(AppColumn::DeletedAt.eq(0_u64))
        .into_model::<ID>()
        .one(master())
        .await?;
    Ok(entiy.is_some())
}
