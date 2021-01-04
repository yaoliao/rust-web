CREATE DATABASE IF NOT EXISTS rust_web
    DEFAULT CHARSET utf8
    COLLATE utf8_general_ci;

-- ----------------------------
-- Table structure for users
-- ----------------------------
USE rust_web;
DROP TABLE IF EXISTS `user`;
CREATE TABLE `user`
(
    `id`       int(11)      NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `username` varchar(255) NOT NULL COMMENT '名字',
    `password` varchar(255) NOT NULL COMMENT '密码',
    `age`      int          NOT NULL COMMENT '年龄'
) ENGINE = InnoDB
    COMMENT = '用户表'
  DEFAULT CHARSET = utf8
  AUTO_INCREMENT = 1;
