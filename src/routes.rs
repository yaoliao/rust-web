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

#[post("/updateAge/<name>/<age>")]
pub fn update_by_name(conn: DbConn, name: String, age: i32) -> Json<Value> {
    let result = User::update_age_by_name(name.as_str(), age, &conn);
    let message;
    if result as i32 == 1 {
        message = String::from("更新成功!")
    } else {
        message = String::from("更新失败!")
    }
    Json(json!({
        "status": 200,
        "message": message,
    }))
}

#[post("/update", format = "application/json", data = "<new_user>")]
pub fn update(conn: DbConn, new_user: Json<User>) -> Json<Value> {
    let user = new_user.into_inner();
    let result = User::update(user.id, user, &conn);
    let message = match result {
        1 => String::from("更新成功!"),
        _ => String::from("更新失败!"),
    };
    Json(json!({
        "status":200,
        "message": message
    }))
}