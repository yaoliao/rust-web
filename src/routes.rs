use rocket_contrib::json::Json;
use serde_json::Value;

use crate::models::UserData;

use super::db::Conn as DbConn;
use super::models::{NewUser, User};

#[get("/all")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::find_users(&conn),
    }))
}

#[post("/newUser", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::find_users(&conn).first(),
    }))
}