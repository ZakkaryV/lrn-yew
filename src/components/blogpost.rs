use stylist::{css, yew::styled_component};
use yew::{html, props, Component, Context, Html, Properties};

#[derive(Clone, Properties, PartialEq)]
pub struct BlogpostProps {
    pub data: Html,
}

pub struct Blogpost;

impl Component for Blogpost {
    type Message = ();
    type Properties = BlogpostProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={css!(r#"
                padding: 15px 0px;
            "#)}>
                { ctx.props().data.clone() }
            </div>
        }
    }

    // fn rendered(&mut self, ctx: &Context<Self>) {}
}
