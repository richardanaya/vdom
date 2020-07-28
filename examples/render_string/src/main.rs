use vdom::*;

fn main() {
    let html = view! {
      Div.add_class("greeting"){
        Text("Hello World!")
      }
    };
    println!("{}", html.render_to_string());
}
