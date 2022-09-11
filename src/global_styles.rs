use stylist::yew::{styled_component, Global};
use yew::{html, Html};

#[styled_component(GlobalStyles)]
pub fn global_styles() -> Html {
    html! {
        <Global css=r#"
            background-color: #FBF0D9;
            color: #5F4B32;
            font-family: monospace;
            padding: 0px 15px;

            a {
                text-decoration: underline;
            }
        "# />
    }
}
