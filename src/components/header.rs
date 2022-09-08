use stylist::{css, yew::styled_component};
use yew::{html, props, Component, Context, Html, Properties};

#[styled_component(Title)]
pub fn title() -> Html {
    html! {
        <a class={css!(r#"
            font-size: 2em;
        "#)} href="https://zakkary.crypto" target="_blank">
        { "Zakkary Verrilli" }
        </a>
    }
}

pub struct Header;

#[derive(Clone, Properties, PartialEq)]
pub struct HeaderProps;

impl Component for Header {
    type Message = ();
    type Properties = HeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav><Title /></nav>
        }
    }
}
