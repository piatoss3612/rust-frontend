pub mod components;
mod router;
use components::atoms::main_title::{Color, MainTitle};
use components::molecules::form::{Data, Form};
use std::ops::Deref;
use std::str::FromStr;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, Style, StyleSource};
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;

use crate::router::{switch, Route};
use components::atoms::struct_hello::StructHello;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

const STYLE_FILE: &str = include_str!("./main.css");

#[derive(Clone, Debug, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

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

    let main_title_load: Callback<String> = Callback::from(|message: String| {
        log!("MainTitle loaded with message:", message);
    });

    let user_state = use_state(|| User::default());

    let onsubmit = {
        let user_state = user_state.clone();

        Callback::from(move |data: Data| {
            let mut user = user_state.deref().clone();

            user.username = data.username;
            user.favorite_language = data.favorite_language;

            user_state.set(user);
        })
    };

    let first_load = use_state(|| true);

    /*
        1. first_load = true
        2. 'App component created' is logged
        3. first_load = false
        4. 'App component dropped' is logged
        5. 'App component updated' is logged
    */
    use_effect_with(*first_load, move |_| {
        if *first_load {
            // this code will run on component creation
            log!("App component created");
            first_load.set(false);
        } else {
            // this code will run on every update
            log!("App component updated");
        }

        || {
            // this code will run when the component is dropped
            log!("App component dropped");
        }
    });

    html! {
        <ContextProvider<User> context={user_state.deref().clone()}>
            <MainTitle title="Hello World123!!" color={Color::Error} on_load={main_title_load} />
            <p class={css!("color: orange;")}>{ "This is my first Yew app!" }</p>

            if let Some(msg) = message {
                <p>{ msg }</p>
            }

            { list_to_html(items) }

            <Form onsubmit={onsubmit} />

            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>

            <StructHello message="Hello World!!!" />
            <components::molecules::struct_counter::StructCounter />
        </ContextProvider<User>>
    }
}

fn list_to_html(items: Vec<i32>) -> Html {
    html! {
        <ul>
            { for items.iter().map(|i| html! { <li>{ i }</li> }) }
        </ul>
    }
}
