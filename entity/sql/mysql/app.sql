USE pilot;

DROP TABLE IF EXISTS `departments`;

CREATE TABLE `departments` (
    `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT '主键,部门ID',
    `name` varchar(64) NOT NULL DEFAULT '' COMMENT '部门名称',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_name` (`name`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '部门';

-- app相关
DROP TABLE IF EXISTS `apps`;

CREATE TABLE `apps` (
    `id` int unsigned AUTO_INCREMENT COMMENT '主键',
    `app` varchar(64) NOT NULL COMMENT 'appID',
    `name` varchar(64) NOT NULL DEFAULT '' COMMENT '应用名',
    `describe` varchar(200) NOT NULL DEFAULT '' COMMENT '详情',
    `department_id` int unsigned NOT NULL DEFAULT 0 COMMENT '所属部门ID',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    primary key (`id`),
    unique key `uk_app_id` (`app`),
    KEY `ix_dept_id` (`dept_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '应用';

-- 用户应用收藏
DROP TABLE IF EXISTS `collection`;

CREATE TABLE `collection` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `user_id` int unsigned NOT NULL COMMENT '用户ID',
    `app_id` int unsigned NOT NULL COMMENT '应用ID',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    primary key (`id`),
    unique key `uk_user_app` (`user_id`, `app_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '应用收藏';

-- app 集群环境
DROP TABLE IF EXISTS `cluster`;

CREATE TABLE `cluster` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `app` varchar(64) NOT NULL COMMENT 'app',
    `cluster` varchar(64) NOT NULL COMMENT '集群',
    `describe` varchar(200) NOT NULL DEFAULT '' COMMENT '详情',
    `secret` varchar(36) NOT NULL COMMENT '密钥',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    primary key (`id`),
    unique key `uk_app_cluster` (`app`, `cluster`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '集群环境';

DROP TABLE IF EXISTS `namespace`;

CREATE TABLE `namespace` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `app` varchar(64) NOT NULL COMMENT 'app',
    `cluster` varchar(64) NOT NULL DEFAULT 'global' COMMENT '集群环境',
    `namespace` varchar(64) NOT NULL COMMENT '命名空间',
    `scope` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '范围',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    primary key (`id`),
    unique key `uk_app_cluster_namespace` (`app`, `cluster`, `namespace`),
    KEY `namespace` (`namespace`, `scope`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '命名空间';

DROP TABLE IF EXISTS `app_extend`;

CREATE TABLE `app_extend` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `app` varchar(64) NOT NULL COMMENT 'appID',
    `namespace_id` bigint unsigned NOT NULL COMMENT '关联的 namespace_id',
    `namespace` varchar(64) NOT NULL COMMENT '命名空间',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_app_namespace` (`app`, `namespace`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = 'app 关联额外 namespace';

DROP TABLE IF EXISTS `item`;

CREATE TABLE `item` (
    `id` bigint unsigned AUTO_INCREMENT COMMENT '主键',
    `namespace_id` bigint unsigned NOT NULL COMMENT '关联的 namespace_id',
    `key` varchar(64) NOT NULL COMMENT '配置key',
    `value` text NOT NULL COMMENT '配置value',
    `category` varchar(20) NOT NULL COMMENT 'value 类型',
    `remark` varchar(200) NOT NULL COMMENT '备注',
    `version` bigint unsigned NOT NULL DEFAULT 0 COMMENT '版本',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `ix_namespace` (`namespace_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '配置表';

DROP TABLE IF EXISTS `release`;

CREATE TABLE `release` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键,发布version',
    `namespace_id` bigint unsigned NOT NULL COMMENT '命名空间ID',
    `key` varchar(64) NOT NULL DEFAULT '' COMMENT '发布的Key',
    `name` varchar(64) NOT NULL DEFAULT '' COMMENT '发布名字',
    `remark` varchar(200) DEFAULT NULL DEFAULT '' COMMENT '发布说明',
    `configurations` mediumtext NOT NULL COMMENT '发布配置',
    `is_abandoned` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '是否废弃',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `ix_namespace` (`namespace_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '发布';

DROP TABLE IF EXISTS `release_history`;

CREATE TABLE `release_history` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `namespace_id` bigint unsigned NOT NULL COMMENT 'namespace',
    `release_version` bigint unsigned NOT NULL COMMENT 'release_version',
    `change` text NOT NULL COMMENT '变更集',
    `status` tinyint unsigned NOT NULL DEFAULT 0 COMMENT '状态',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `ix_namespace` (`namespace_id`, `status`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '发布';