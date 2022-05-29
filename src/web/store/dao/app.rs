use std::str::FromStr;

use super::master;
use crate::web::extract::jwt::Claims;

use entity::orm::{
    ColumnTrait, DbErr, EntityTrait, Iterable, QueryFilter, QuerySelect, Set, TransactionError,
    TransactionTrait,
};
use entity::rule::Verb;
use entity::{
    AppActive, AppColumn, AppEntity, AppModel, IDu32, RoleActive, RoleEntity, RoleRuleActive,
    RoleRuleEntity, RuleActive, RuleEntity, UserRoleActive, UserRoleEntity, ID,
};

pub async fn add(app_id: String, name: String, auth: &Claims) -> Result<(), DbErr> {
    // 构造应用数据
    let app = AppActive {
        app_id: Set(app_id.clone()),
        name: Set(name),
        org_id: Set(auth.org_id),
        creator_user: Set(auth.user_id),
        ..Default::default()
    };
    // 构造默认角色
    let role = RoleActive {
        name: Set(String::from_str("Master/").unwrap() + &app_id),
        ..Default::default()
    };
    // 构造用户角色
    let mut user_bind = UserRoleActive {
        user_id: Set(auth.user_id),
        ..Default::default()
    };
    let verb_len = Verb::iter().len();
    // 构造权限
    let mut rules: Vec<RuleActive> = Vec::with_capacity(verb_len);
    for v in Verb::iter() {
        let rule = RuleActive {
            verb: Set(v),
            resource: Set(app_id.clone()),
            ..Default::default()
        };
        rules.push(rule);
    }
    let mut binds: Vec<RoleRuleActive> = Vec::with_capacity(verb_len);

    let transaction = master()
        .transaction::<_, (), DbErr>(|tx| {
            Box::pin(async move {
                // 创建应用
                let _ = AppEntity::insert(app).exec(tx).await?;
                // 创建角色
                let role_id = RoleEntity::insert(role).exec(tx).await?.last_insert_id;
                // 创建权限
                let rule_id = RuleEntity::insert_many(rules)
                    .exec(tx)
                    .await?
                    .last_insert_id;
                let last_rule_id = rule_id as usize + verb_len;
                // 角色绑定权限
                for id in rule_id as usize..last_rule_id {
                    let bind = RoleRuleActive {
                        role_id: Set(role_id),
                        rule_id: Set(id as u64),
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
        .await;
    if let Err(e) = transaction {
        match e {
            TransactionError::Connection(err) => {
                return Err(err);
            }
            TransactionError::Transaction(err) => {
                return Err(DbErr::Exec(err.to_string()));
            }
        }
    }
    Ok(())
}

pub async fn find_all(offset: u64, limit: u64) -> Result<Vec<AppModel>, DbErr> {
    AppEntity::find()
        .offset(offset)
        .limit(limit)
        .all(master())
        .await
}

// 查找 app_id 是否存在
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
