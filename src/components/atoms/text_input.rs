use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_on_change: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_on_change = props.handle_on_change.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().expect("Could not get event target");
        let input = target.unchecked_into::<HtmlInputElement>();

        handle_on_change.emit(input.value());
    });

    html! {
        <input type="text" name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()} />
    }
}
