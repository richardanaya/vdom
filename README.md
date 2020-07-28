# vdom

An elegant vdom inspired by SwiftUI.

```rust
use vdom::*;


pub struct TodoItem;

impl TodoItem {
  fn new(s:&str) -> VNode{
    view! {
      Div.add_class("todo-item"){
        Text(s)
      }
    }
  }
}

pub struct TodoList;

impl TodoList {
  fn new(todos:Vec<String>) -> VNode{
    view! {
      Div.add_class("todo-list"){
        For(i in todos.iter()) { TodoItem(i) }
      }
    }
  }
}

fn main() {
    let todos = vec![
      "drink coffee".to_string(),
      "write some code".to_string(),
      "eat lunch".to_string(),
    ];

    let list = TodoList::new(todos);

    println!("{}", list.render_to_string());
}
```