use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
  pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo-list")]
pub struct TodoList {
  pub id: i32,
  pub title: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo-item")]
pub struct TodoItem {
  pub id: i32,
  pub title: String,
  pub checked: bool,
  pub list_id: i32,
}
