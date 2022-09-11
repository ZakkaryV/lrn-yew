use std::{error::{Error,}, fmt::{self, Debug, Display, Formatter}};
use crate::console_log;
use crate::components::blogpost::Blogpost;
use crate::components::header::Header;
use crate::components::solana_connect::{create_solana_connection, SolanaConnectionProvider, SolanaConnectionProviderProps};
use crate::global_styles::GlobalStyles;
use js_sys::Reflect;
use web_sys::{HtmlInputElement, Request, Response, RequestInit, RequestMode, console::log};
use wasm_bindgen::{JsValue, JsCast};
use yew::{classes, html, props, Children, Component, Context, Html, NodeRef, Properties, function_component, use_state_eq, Callback};


#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Error for FetchError {}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

// impl From<JsValue> for JsError {
//     fn from(value: JsValue) -> Self {
//         Self {
//             err: value,
//         }
//     }
// }

pub enum FetchState<T> {
    NotFetching,
    Fetching, 
    Success(T),
    Failed(FetchError),
}

pub enum Msg {
    SetFetchingState(FetchState<JsValue>),
    GetData,
    GetError,
}

#[derive(Clone, PartialEq, Properties, Default)]
pub struct Props {}

pub struct AppComponent {
    node_ref: NodeRef,
    data: FetchState<JsValue>,
}

impl Component for AppComponent {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchingState(fetch_state) => {
                console_log(String::from("1"));
                self.data = fetch_state;
                true
            },
            Msg::GetData => {
                ctx.link().send_future(async {
                console_log(String::from("2"));
                    match crate::utils::fetch::fetch(String::from("zakkvry/lrn-yew")).await {
                        Ok(text) => Msg::SetFetchingState(FetchState::Success(text.clone().into())),
                        Err(err) => Msg::SetFetchingState(FetchState::Failed(FetchError { err: err.clone().into()})),
                    }
                });
                console_log(String::from("3"));

                ctx.link().send_future(async {
                    match create_solana_connection().await {
                        Ok(data) => Msg::SetFetchingState(FetchState::Success(JsValue::from(data))),
                        Err(err) => Msg::SetFetchingState(FetchState::Failed(FetchError { err: JsValue::from("error")}))
                    }
                });

                ctx.link().send_message(Msg::SetFetchingState(FetchState::Fetching));
                false
            },
            Msg::GetError => {
                false
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            data: FetchState::NotFetching,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let blogposts_data: Html = match &self.data {
             FetchState::NotFetching => {
                 ctx.link().send_message(Msg::GetData);
                 html! {"Loading..."}
             },
             FetchState::Fetching => {
                 html! {"Loading..."}
             },
             FetchState::Success(data) => {
                 let property_name = JsValue::from("name");
                 let s = Reflect::get(&data, &property_name).unwrap();
                 html! {format!("branch name: {:#?}", s)}
             },
             FetchState::Failed(error) => {
                 html! {format!("Get wrecked. {:#?}", error)}
             },
         };

        html! {
            <div>
                <GlobalStyles />
                <Header />
                <Blogpost data={blogposts_data} />
                <SolanaConnectionProvider />
                // { ctx.props().children.clone() }
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                input.focus();
            }
        }
    }
}
