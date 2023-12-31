use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Hello)]
pub fn hello() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <h1>{"Hello"}</h1>
            <button onclick={onclick}>{"Go to Home"}</button>
        </div>
    }
}
