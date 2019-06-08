#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::State;

use std::sync::RwLock;

struct TodoConfig {
  todos: RwLock<Vec<String>>
}

#[get("/<name>")]
fn list(state: State<TodoConfig>, name: String) -> String {
  let todos = state.todos.read().unwrap();
  format!("Hello {}, here are the todos: \n- {}", name.as_str(), todos.join("\n- "))
}

#[post("/add", data = "<todo_name>")]
fn add_todo(state: State<TodoConfig>, todo_name: String) -> String {
  state.todos.write().unwrap().push(todo_name.to_string());

  format!("\"{}\" has been added", todo_name)
}

fn main() {
  let config = TodoConfig {
    todos: RwLock::new(vec![])
  };
  rocket::ignite()
  .mount("/", routes![
    list,
    add_todo
  ])
  .manage(config)
  .launch();
}
