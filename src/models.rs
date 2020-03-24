use serde::{Deserialize, Serialize};

//TODOのモデルを定義
#[derive(Debug, Serialize,  Deserialize)]
pub struct ToDo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub done: bool,
}