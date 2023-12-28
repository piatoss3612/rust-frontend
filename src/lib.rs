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

    let class = "my-title";
    let message: Option<String> = Some("Hello World!!".to_string());

    let items = (1..=5).collect::<Vec<i32>>();

    html! {
        <div>
            <h1 class={class}>{ "Hello World!!" }</h1>
            if class == "my-title" {
                <p>{ "This is my first Yew app!" }</p>
            } else {
                <p>{ "This is not my first Yew app!" }</p>
            }

            if let Some(msg) = message {
                <p>{ msg }</p>
            }

            { list_to_html(items) }
        </div>
    }
}

fn list_to_html(items: Vec<i32>) -> Html {
    html! {
        <ul>
            { for items.iter().map(|i| html! { <li>{ i }</li> }) }
        </ul>
    }
}
