use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;

#[function_component(Form)]
pub fn form() -> Html {
    let username_state = use_state(|| "".to_string());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username: String| {
        cloned_username_state.set(username);
    });

    html!(
        <form>
            <TextInput name="username" handle_on_change={username_changed} />
            <CustomButton label="Submit" />
            <p>{ &*username_state} </p>
        </form>
    )
}
