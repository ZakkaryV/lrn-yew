use js_sys::Reflect;
use web_sys::{HtmlInputElement, Request, Response, RequestInit, RequestMode};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn fetch(repo: String) -> Result<JsValue, JsValue> {
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
