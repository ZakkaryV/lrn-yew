use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;
mod components;
mod global_styles;
mod utils;
use app::AppComponent;

#[wasm_bindgen]
extern "C" {
    // define an imported JS function for use in RS
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: String);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);
}

#[wasm_bindgen]
pub fn console_log(s: String) -> () {
    let a = js_sys::Array::new();
    a.push(&s.into());
    let rust_string: String = a.to_string().into();
    log(rust_string);
}

fn main() {
    yew::start_app::<AppComponent>();
}
