CREATE DATABASE IF NOT EXISTS pilot DEFAULT CHARACTER SET = utf8mb4;

USE pilot;

-- user 相关
DROP TABLE IF EXISTS `users`;

CREATE TABLE `users` (
    `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `account` varchar(64) NOT NULL COMMENT '用户登录账户',
    `email` varchar(64) NOT NULL DEFAULT '' COMMENT '邮箱地址',
    `nickname` varchar(32) NOT NULL DEFAULT '' COMMENT '用户名称',
    `password` varchar(512) NOT NULL COMMENT '密码',
    `dept_id` int unsigned NOT NULL DEFAULT 0 COMMENT '所属部门ID',
    `level` smallint unsigned NOT NULL DEFAULT 0 COMMENT '管理员等级',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_account` (`account`),
    UNIQUE KEY `uk_email` (`email`),
    key `ix_dept_id` (`dept_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '用户';

DROP TABLE IF EXISTS `role`;

CREATE TABLE `role` (
    `id` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
    `name` varchar(32) NOT NULL DEFAULT '' COMMENT 'Role name',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_name_delete` (`name`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '角色';

DROP TABLE IF EXISTS `user_role`;

CREATE TABLE `user_role` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `user_id` int unsigned NOT NULL DEFAULT 0 COMMENT '用户身份标识',
    `role_id` int unsigned NOT NULL DEFAULT 0 COMMENT 'Role Id',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_user_role_id` (`user_id`, `role_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '用户和角色的绑定表';

DROP TABLE IF EXISTS `rule`;

CREATE TABLE `rule` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `verb` varchar(32) NOT NULL DEFAULT '' COMMENT '权限类型',
    `resource` varchar(200) NOT NULL DEFAULT '' COMMENT '资源',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_resource_verb` (`resource`, `verb`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '权限';

DROP TABLE IF EXISTS `role_rule`;

CREATE TABLE `role_rule` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `role_id` int unsigned NOT NULL COMMENT 'Role Id',
    `rule_id` bigint unsigned NOT NULL COMMENT 'Rule Id',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_rule_role_id` (`rule_id`, `role_id`),
    KEY `ix_role_rule_id` (`role_id`, `rule_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '角色和权限的绑定表';
