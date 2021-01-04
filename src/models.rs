use diesel;
use diesel::{MysqlConnection, RunQueryDsl};
use diesel::prelude::*;

use crate::schema::user as user;

use super::schema::user::dsl::user as all_users;

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
        all_users
            .order(user::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(new_user: NewUser, conn: &MysqlConnection) -> bool {
        diesel::insert_into(user::table)
            .values(&new_user)
            .execute(conn)
            .is_ok()
    }
}