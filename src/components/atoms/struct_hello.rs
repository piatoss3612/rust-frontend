use stylist::{style, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub message: String,
}

pub struct StructHello {
    pub stylesheet: Style,
}

impl StructHello {
    pub fn style() -> Style {
        style!(
            r#"
            color: green;
        "#
        )
        .unwrap()
    }
}

impl Component for StructHello {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stylesheet: Self::style(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();

        html! {
            <div>
                <h1 class={self.stylesheet.clone()}>{props.message}</h1>
            </div>
        }
    }
}
