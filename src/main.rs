#![feature(proc_macro_hygiene, decl_macro)]

/**
 * If you have not read the README.md I would recommend doing that first!
 * It is different between this branch and the master branch
 */

// rocket is the rest api, https://rocket.rs/
#[macro_use]
extern crate rocket;
use rocket::State;

// RwLock is a way of wrapping a variable in a write mutex but
// allowing as many readers as requested provided no one is writing
// see more here: https://doc.rust-lang.org/std/sync/struct.RwLock.html
use std::sync::RwLock;

// This struct will be used as the state configuration for rocket
struct TodoConfig {
  // Lock the todos so multiple add endpoints don't modify it at the same time
  todos: RwLock<Vec<String>>
}

// create the GET /{name} endpoint. The name will be used in the response
// I would like to make it so you can map names to todos in the future
#[get("/<name>")]
fn list(state: State<TodoConfig>, name: String) -> String {
  let todos = state.todos.read().unwrap();

  // return a message listing all the todos
  format!("Hello {}, here are the todos: \n- {}", name.as_str(), todos.join("\n- "))
}

// create the /add POST endpoint, any data body will be read as a string and added as a todo
// I would like to make this JSON in the future
#[post("/add", data = "<todo_name>")]
fn add_todo(state: State<TodoConfig>, todo_name: String) -> String {
  state.todos.write().unwrap().push(todo_name.to_string());

  // return a message stating that we've added the todo
  format!("\"{}\" has been added", todo_name)
}

fn main() {
  // create the rocket state config
  let config = TodoConfig {
    todos: RwLock::new(vec![])
  };

  // start the rest api
  rocket::ignite()
  .mount("/", routes![
    list,
    add_todo
  ])
  .manage(config)
  .launch();
}
