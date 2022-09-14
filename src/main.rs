mod app;
mod components;
mod global_styles;
mod utils;

use app::AppComponent;
use wasm_bindgen::prelude::*;
use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<AppComponent>();
}
