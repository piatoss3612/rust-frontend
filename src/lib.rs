use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "Bob";
    let my_object = MyObject {
        username: "Bob".to_string(),
        favorite_language: "Rust".to_string(),
    };

    log!("Hello,", name, "!");
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    html! {
        <div>
            <h1>{ "Hello World!!" }</h1>
            <p>{ "This is my first Yew app!" }</p>
        </div>
    }
}
