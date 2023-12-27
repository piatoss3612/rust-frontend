use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello World!!" }</h1>
            <p>{ "This is my first Yew app!" }</p>
        </div>
    }
}
