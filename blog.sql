/*
 Navicat Premium Dump SQL

 Source Server         : MySQL
 Source Server Type    : MySQL
 Source Server Version : 80403 (8.4.3)
 Source Host           : localhost:3306
 Source Schema         : blog

 Target Server Type    : MySQL
 Target Server Version : 80403 (8.4.3)
 File Encoding         : 65001

 Date: 28/01/2025 10:58:45
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for admin
-- ----------------------------
DROP TABLE IF EXISTS `admin`;
CREATE TABLE `admin` (
  `id` varchar(128) NOT NULL,
  `name` varchar(16) NOT NULL COMMENT '管理员名称',
  `password` blob NOT NULL COMMENT '密码',
  `nickname` varchar(16) DEFAULT NULL COMMENT '昵称',
  `email` varchar(128) DEFAULT NULL COMMENT '邮箱',
  `github` varchar(255) DEFAULT NULL COMMENT 'Github',
  `wechat` varchar(255) DEFAULT NULL COMMENT '微信',
  `qq` varchar(255) DEFAULT NULL COMMENT 'QQ',
  `avatar` varchar(255) DEFAULT NULL COMMENT '头像',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='管理员表';

-- ----------------------------
-- Table structure for article
-- ----------------------------
DROP TABLE IF EXISTS `article`;
CREATE TABLE `article` (
  `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT '文章ID',
  `title` varchar(128) NOT NULL COMMENT '文章标题',
  `summary` varchar(255) DEFAULT NULL COMMENT '文章摘要',
  `content` text NOT NULL COMMENT '文章内容',
  `views` int unsigned NOT NULL DEFAULT '0' COMMENT '浏览量',
  `publish` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否发布',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='文章表';

-- ----------------------------
-- Table structure for article_category
-- ----------------------------
DROP TABLE IF EXISTS `article_category`;
CREATE TABLE `article_category` (
  `article_id` int unsigned NOT NULL COMMENT '对应文章ID',
  `category_id` int unsigned NOT NULL COMMENT '对应分类ID',
  PRIMARY KEY (`article_id`,`category_id`),
  KEY `fk-article_category-category` (`category_id`),
  CONSTRAINT `fk-article_category-article` FOREIGN KEY (`article_id`) REFERENCES `article` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `fk-article_category-category` FOREIGN KEY (`category_id`) REFERENCES `category` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='文章分类关联表';

-- ----------------------------
-- Table structure for article_tag
-- ----------------------------
DROP TABLE IF EXISTS `article_tag`;
CREATE TABLE `article_tag` (
  `article_id` int unsigned NOT NULL COMMENT '对应文章ID',
  `tag_id` int unsigned NOT NULL COMMENT '对应标签ID',
  PRIMARY KEY (`article_id`,`tag_id`),
  KEY `fk-article_tag-tag` (`tag_id`),
  CONSTRAINT `fk-article_tag-article` FOREIGN KEY (`article_id`) REFERENCES `article` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `fk-article_tag-tag` FOREIGN KEY (`tag_id`) REFERENCES `tag` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='文章标签关联表';

-- ----------------------------
-- Table structure for category
-- ----------------------------
DROP TABLE IF EXISTS `category`;
CREATE TABLE `category` (
  `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT '分类ID',
  `name` varchar(32) NOT NULL COMMENT '分类名称',
  `parent_id` int unsigned DEFAULT NULL COMMENT '父分类',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`),
  KEY `fk-category-parent` (`parent_id`),
  CONSTRAINT `fk-category-parent` FOREIGN KEY (`parent_id`) REFERENCES `category` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='分类表';

-- ----------------------------
-- Table structure for comment
-- ----------------------------
DROP TABLE IF EXISTS `comment`;
CREATE TABLE `comment` (
  `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT '评论ID',
  `article` int unsigned NOT NULL COMMENT '评论所在文章ID',
  `name` varchar(32) NOT NULL COMMENT '评论者名称',
  `email` varchar(255) DEFAULT NULL,
  `parent_id` int unsigned NOT NULL COMMENT '父评论ID',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `fk-comment-article` (`article`),
  KEY `fk-comment-parent` (`parent_id`),
  CONSTRAINT `fk-comment-article` FOREIGN KEY (`article`) REFERENCES `article` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `fk-comment-parent` FOREIGN KEY (`parent_id`) REFERENCES `comment` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='评论表';

-- ----------------------------
-- Table structure for jwt
-- ----------------------------
DROP TABLE IF EXISTS `jwt`;
CREATE TABLE `jwt` (
  `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `user_uuid` varchar(128) NOT NULL COMMENT '用户UUID',
  `token` varchar(512) NOT NULL COMMENT 'token',
  `expire_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '过期时间',
  `expire_duration` bigint NOT NULL COMMENT '过期时长, 单位秒',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `fk-jwt-user` (`user_uuid`),
  CONSTRAINT `fk-jwt-user` FOREIGN KEY (`user_uuid`) REFERENCES `admin` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='存放JWT, 需要定时清理';

-- ----------------------------
-- Table structure for seaql_migrations
-- ----------------------------
DROP TABLE IF EXISTS `seaql_migrations`;
CREATE TABLE `seaql_migrations` (
  `version` varchar(255) NOT NULL,
  `applied_at` bigint NOT NULL,
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- ----------------------------
-- Table structure for tag
-- ----------------------------
DROP TABLE IF EXISTS `tag`;
CREATE TABLE `tag` (
  `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT '标签ID',
  `name` varchar(32) NOT NULL COMMENT '标签名称',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='标签表';

-- ----------------------------
-- Table structure for website
-- ----------------------------
DROP TABLE IF EXISTS `website`;
CREATE TABLE `website` (
  `title` varchar(32) NOT NULL COMMENT '网站标题',
  `subtitle` varchar(64) DEFAULT NULL COMMENT '网站副标题',
  `description` varchar(64) DEFAULT NULL COMMENT '网站描述',
  `logo` varchar(255) DEFAULT NULL COMMENT '网站 Logo',
  `favicon` varchar(255) DEFAULT NULL COMMENT '网站 Favicon',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`title`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='网站基本信息';

SET FOREIGN_KEY_CHECKS = 1;
