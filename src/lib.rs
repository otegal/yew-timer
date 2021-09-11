extern crate console_error_panic_hook;
use std::panic;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    time: String,
}

impl Model {
    fn current_time() -> String {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("JP"))
    }
}

enum Msg {
    UpdateTime,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            time: Model::current_time(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTime => self.time = Model::current_time(),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <button onclick=self.link.callback(|_| Msg::UpdateTime)> { "update time" } </button>
                { &self.time }
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
