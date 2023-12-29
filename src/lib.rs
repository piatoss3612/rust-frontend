pub mod components;
use components::atoms::main_title::{Color, MainTitle};

use std::str::FromStr;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, Style, StyleSource};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

const STYLE_FILE: &str = include_str!("./main.css");

#[styled_component(App)]
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

    // let stylesheet = style!(
    //     r#"
    //         color: red;
    //         font-size: 48px;
    // "#
    // )
    // .unwrap();

    let css = StyleSource::from_str(STYLE_FILE).unwrap();

    let stylesheet = Style::new(css).unwrap();

    html! {
        <div class={stylesheet}>
            <MainTitle title="Hello World123!!" color={Color::Error} />
            <p class={css!("color: orange;")}>{ "This is my first Yew app!" }</p>

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
