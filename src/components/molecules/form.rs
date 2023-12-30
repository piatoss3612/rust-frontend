use std::ops::Deref;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct Data {
    pub username: String,
    pub favorite_language: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(Form)]
pub fn form(props: &Props) -> Html {
    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username: String| {
        cloned_state.set(Data {
            username,
            ..cloned_state.deref().clone()
        })
    });

    let cloned_state = state.clone();
    let language_changed = Callback::from(move |language: String| {
        cloned_state.set(Data {
            favorite_language: language,
            ..cloned_state.deref().clone()
        })
    });

    let onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let handle_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        onsubmit.emit(data);
    });

    html!(
        <form onsubmit={handle_submit}>
            <TextInput name="username" handle_on_change={username_changed} />
            <TextInput name="favorite_language" handle_on_change={language_changed} />
            <CustomButton label="Submit"/>
            <p>{ &state.username} </p>
            <p>{ &state.favorite_language} </p>
        </form>
    )
}
