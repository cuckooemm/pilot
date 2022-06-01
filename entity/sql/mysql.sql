CREATE DATABASE IF NOT EXISTS cmm DEFAULT CHARACTER SET = utf8mb4;

USE cmm;

-- user 相关
DROP TABLE IF EXISTS `users`;

CREATE TABLE `users` (
    `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `account` varchar(64) NOT NULL COMMENT '用户登录账户',
    `email` varchar(64) NOT NULL DEFAULT '' COMMENT '邮箱地址',
    `nickname` varchar(64) NOT NULL DEFAULT '' COMMENT '用户名称',
    `password` varchar(512) NOT NULL COMMENT '密码',
    `dept_id` int unsigned NOT NULL DEFAULT 0 COMMENT '所属部门ID',
    `dept_name` varchar(64) NOT NULL DEFAULT '' COMMENT '所属部门名',
    `level` smallint unsigned NOT NULL DEFAULT 0 COMMENT '管理员等级 100:超级管理员 10:部门管理员',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_account` (`account`),
    UNIQUE KEY `uk_email` (`email`),
    key `ix_dept_id` (`dept_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '用户';

DROP TABLE IF EXISTS `department`;

CREATE TABLE `department` (
    `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT '主键,部门ID',
    `name` varchar(128) NOT NULL DEFAULT '' COMMENT '部门名称',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_name` (`name`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '部门';

DROP TABLE IF EXISTS `role`;

CREATE TABLE `role` (
    `id` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
    `name` varchar(255) NOT NULL DEFAULT '' COMMENT 'Role name',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_name_delete` (`name`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '角色';

DROP TABLE IF EXISTS `user_role`;

CREATE TABLE `user_role` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `user_id` int NOT NULL DEFAULT 0 COMMENT '用户身份标识',
    `role_id` int unsigned NOT NULL COMMENT 'Role Id',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_user_role_id` (`user_id`, `role_id`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '用户和角色的绑定表';

DROP TABLE IF EXISTS `rule`;

CREATE TABLE `rule` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `verb` varchar(32) NOT NULL DEFAULT '' COMMENT '权限类型',
    `resource` varchar(255) NOT NULL DEFAULT '' COMMENT '资源',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_resource_verb` (`resource`, `verb`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '权限';

DROP TABLE IF EXISTS `role_rule`;

CREATE TABLE `role_rule` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `role_id` int unsigned NOT NULL COMMENT 'Role Id',
    `rule_id` bigint unsigned NOT NULL COMMENT 'Rule Id',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_rule_role_id` (`rule_id`, `role_id`, `deleted_at`),
    KEY `ix_role_rule_id` (`role_id`, `rule_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '角色和权限的绑定表';

-- app相关
DROP TABLE IF EXISTS `app`;

CREATE TABLE `app` (
    `id` int unsigned AUTO_INCREMENT COMMENT '主键',
    `app_id` varchar(80) NOT NULL COMMENT 'appID',
    `name` varchar(100) NOT NULL COMMENT '应用名',
    `dept_id` int unsigned NOT NULL DEFAULT 0 COMMENT '所属部门ID',
    `creator_user` int unsigned DEFAULT 0 NOT NULL COMMENT '创建用户ID',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    primary key (`id`),
    unique key `uk_app_id` (`app_id`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '应用';

-- 用户应用收藏
DROP TABLE IF EXISTS `user_favorite`;

CREATE TABLE `user_favorite` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `user_id` int unsigned NOT NULL COMMENT '用户ID',
    `app_id` int unsigned NOT NULL COMMENT '应用ID',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    primary key (`id`),
    unique key `uk_user_app` (`user_id`, `app_id`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '应用收藏';

-- app 集群环境
DROP TABLE IF EXISTS `cluster`;

CREATE TABLE `cluster` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `app_id` varchar(80) NOT NULL COMMENT 'appID',
    `name` varchar(80) NOT NULL COMMENT '集群环境',
    `secret` varchar(36) NOT NULL COMMENT '密钥',
    `creator_user` int unsigned NOT NULL COMMENT '创建的用户ID',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    primary key (`id`),
    unique key `uk_app_cluster` (`app_id`, `name`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '集群环境';

DROP TABLE IF EXISTS `namespace`;

CREATE TABLE `namespace` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `app_id` varchar(80) NOT NULL COMMENT 'appID',
    `cluster` varchar(80) NOT NULL DEFAULT 'global' COMMENT '集群环境',
    `namespace` varchar(80) NOT NULL COMMENT '命名空间',
    `scope` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '范围',
    `creator_user` int unsigned NOT NULL COMMENT '创建的用户ID',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    primary key (`id`),
    unique key `uk_app_cluster_namespace` (`app_id`, `cluster`, `namespace`, `deleted_at`),
    KEY `namespace` (`namespace`, `scope`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '命名空间';

DROP TABLE IF EXISTS `app_extend`;

CREATE TABLE `app_extend` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `app_id` varchar(80) NOT NULL COMMENT 'appID',
    `namespace_id` bigint unsigned NOT NULL COMMENT '关联的 namespace_id',
    `namespace_name` varchar(80) NOT NULL COMMENT '命名空间',
    `creator_user` int unsigned NOT NULL COMMENT '创建的用户ID',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_app_namespace` (`app_id`, `namespace_name`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '应用namespace定义';

DROP TABLE IF EXISTS `item`;

CREATE TABLE `item` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `namespace_id` bigint unsigned NOT NULL COMMENT '关联的 namespace_id',
    `key` varchar(255) NOT NULL COMMENT '配置key',
    `value` text NOT NULL COMMENT '配置value',
    `category` varchar(20) NOT NULL COMMENT 'value 类型',
    `remark` varchar(255) NOT NULL COMMENT '备注',
    `version` bigint unsigned NOT NULL DEFAULT 0 COMMENT '版本',
    `modify_user_id` int unsigned NOT NULL DEFAULT 0 COMMENT '最后修改用户',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `ix_namespace` (`namespace_id`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '配置表';

DROP TABLE IF EXISTS `release`;

CREATE TABLE `release` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键,发布version',
    `namespace_id` bigint unsigned NOT NULL COMMENT '命名空间ID',
    `key` varchar(64) NOT NULL DEFAULT '' COMMENT '发布的Key',
    `name` varchar(64) NOT NULL DEFAULT '' COMMENT '发布名字',
    `remark` varchar(255) DEFAULT NULL DEFAULT '' COMMENT '发布说明',
    `configurations` longtext NOT NULL COMMENT '发布配置',
    `is_abandoned` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '是否废弃',
    `publish_user_id` int NOT NULL DEFAULT 0 COMMENT '用户身份标识',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `ix_namespace` (`namespace_id`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '发布';

DROP TABLE IF EXISTS `release_history`;

CREATE TABLE `release_history` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键',
    `namespace_id` bigint unsigned NOT NULL COMMENT '命名空间ID',
    `change` longtext NOT NULL COMMENT '变更集',
    `release_id` bigint unsigned NOT NULL COMMENT '对应release_id',
    `deleted_at` bigint unsigned NOT NULL DEFAULT 0 COMMENT '删除时间 second',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `ix_namespace` (`namespace_id`, `deleted_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '发布';