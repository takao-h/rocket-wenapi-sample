// JSONを返すのに必要
use rocket_contrib::json::Json;

use crate::models::*;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

/// TODOリストを返す。
#[get("/todos")]
pub fn todos() -> Json<Vec<ToDo>> {
    Json(vec![ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    }])
}

/// 新しいTODOを作成する
/// POSTの時はこうする
#[post("/todos", data = "<todo>")]
pub fn new_todo(todo: Json<ToDo>) -> String {
    format!("Accepted post request! {:?}", todo.0)
}

/// TODOを取得する
#[get("/todos/<todoid>")]
pub fn todo_by_id(todoid: u32) -> String {
    let todo = ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    };
    format!("{:?}", todo)
}

/// user一覧を取得する
#[get("/users")]
pub fn users() -> Json<Vec<User>> {
    Json(vec! [User {
        id: 1,
        name: "takao".into(),
        email: "hoge@email.com".into(),
    }])
}