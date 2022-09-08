use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;
mod components;
use app::AppComponent;

#[wasm_bindgen]
extern "C" {
    // define an imported JS function for use in RS
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello hello hello woorldld {}", name));
}

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                // greet(&format!("NEW VALUE IS: {}", self.value + 1));
                // self.value += 1;
                // returning true tells the tree to re-render children due to a change at this node
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <AppComponent />
                // <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                // <p>{ self.value }</p>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
