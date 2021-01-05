use diesel;
use diesel::{MysqlConnection, RunQueryDsl};
use diesel::debug_query;
use diesel::mysql::Mysql;
use diesel::prelude::*;

use crate::schema::user as user;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub age: i32,
}

// decode request data
#[derive(Deserialize)]
pub struct UserData {
    pub username: String
}

// this is to insert user to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub age: i32,
}

impl User {
    pub fn find_users(conn: &MysqlConnection) -> Vec<User> {
        let query = user::table
            .order(user::id.desc());
        // 打印 sql
        println!("{}", debug_query::<Mysql, _>(&query));
        query.load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(new_user: NewUser, conn: &MysqlConnection) -> bool {
        diesel::insert_into(user::table)
            .values(&new_user)
            .execute(conn)
            .is_ok()
    }

    pub fn update_age_by_name(name: &str, age: i32, conn: &MysqlConnection) -> usize {
        use crate::schema::user::dsl::user as dsl_user;
        let filter = dsl_user.filter(user::username.eq(name));
        let update = diesel::update(filter).set(user::age.eq(age));
        println!("{}", debug_query::<Mysql, _>(&update));
        update.execute(conn)
            .unwrap()
    }

    pub fn update(id: i32, new_user: User, conn: &MysqlConnection) -> usize {
        use crate::schema::user::dsl::user as dsl_user;
        let filter = dsl_user.filter(user::id.eq(id));
        diesel::update(filter).set(
            (user::username.eq(new_user.username),
             user::password.eq(new_user.password),
             user::age.eq(new_user.age)))
            .execute(conn).unwrap()
    }
}