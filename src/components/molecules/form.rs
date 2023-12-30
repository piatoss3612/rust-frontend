use std::ops::Deref;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;

#[derive(Default, Debug, Clone)]
struct Data {
    pub username: String,
    pub count: u32,
}

#[function_component(Form)]
pub fn form() -> Html {
    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username: String| {
        cloned_state.set(Data {
            username,
            ..cloned_state.deref().clone()
        })
    });

    let cloned_state = state.clone();
    let button_changed = Callback::from(move |_| {
        cloned_state.set(Data {
            count: cloned_state.count + 1,
            ..cloned_state.deref().clone()
        })
    });

    html!(
        <div>
            <TextInput name="username" handle_on_change={username_changed} />
            <CustomButton label="Submit" onclick={button_changed}/>
            <p>{ &state.username} </p>
            <p>{"Button clicked: "}{ state.count }</p>
        </div>
    )
}
