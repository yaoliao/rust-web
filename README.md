#### rust web 例子

##### 遇到的问题
1、diesel_cli 编译报错
> note: ld: library not found for -lmysqlclient

解决办法：https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md

2、'diesel setup' 报错
> Can't connect to local MySQL server through socket '/tmp/mysql.sock' (2)

解决办法：https://github.com/diesel-rs/diesel/issues/2171

##### 参考
[Rust ORM 框架 Diesel](https://www.rectcircle.cn/posts/rust-diesel/)

[rocket+diesel+mysql学习](https://www.jianshu.com/p/95452dbe343b)