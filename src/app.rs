use std::{error::{Error,}, fmt::{self, Debug, Display, Formatter}};
use crate::components::blogpost::Blogpost;
use crate::components::header::Header;
use stylist::yew::{styled_component, Global};
use js_sys::Reflect;
use web_sys::{HtmlInputElement, Request, Response, RequestInit, RequestMode, console::log};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use yew::{classes, html, props, Children, Component, Context, Html, NodeRef, Properties};
use crate::solana_connect::create_solana_connection;


pub fn console_log(s: String) -> () {
    let a = js_sys::Array::new();
    a.push(&s.into());
    log(&a)
}

#[wasm_bindgen]
pub async fn fetch_data(repo: String) -> Result<JsValue, JsValue> {
    let url = format!("https://api.github.com/repos/{}/branches/master", repo);
    let mut opts = RequestInit::new();

    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();

    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(response_value.is_instance_of::<Response>());

    let response: Response = response_value.dyn_into().unwrap();

    let json = JsFuture::from(response.json()?).await?;

    Ok(json)
}


#[styled_component(Stylewrapper)]
pub fn stylewrapper() -> Html {
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

#[derive(Clone, PartialEq, Properties)]
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
                    match fetch_data(String::from("zakkvry/lrn-yew")).await {
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
                <Stylewrapper />
                <Header />
                <Blogpost data={blogposts_data} />
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
