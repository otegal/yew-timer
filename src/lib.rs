extern crate console_error_panic_hook;
use std::panic;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{IntervalService, ConsoleService, Task, TimeoutService};
use std::time::Duration;

struct Model {
    link: ComponentLink<Self>,
    job: Option<Box<dyn Task>>,
    time: String,
    messages: Vec<&'static str>
}

impl Model {
    fn current_time() -> String {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("JP"))
    }
    fn current_time_with_suffix (suffix: &str) -> &'static str {
        let now = Model::current_time();
        let description = now + "\t" + suffix;
        Box::leak(description.into_boxed_str())
    }
}

enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
    UpdateTime,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            job: None,
            time: Model::current_time(),
            messages: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartTimeout => {
                let handle = TimeoutService::spawn(
                    Duration::from_secs(3),
                    self.link.callback(|_| Msg::Done),
                );
                self.job = Some(Box::new(handle));

                self.messages.clear();
                ConsoleService::clear();

                self.messages.push(Model::current_time_with_suffix("Timeout started!"));
                ConsoleService::time_named("Timer");
                true
            }
            Msg::StartInterval => {
                let handle = IntervalService::spawn(
                    Duration::from_secs(1),
                    self.link.callback(|_| Msg::Tick),
                );
                self.job = Some(Box::new(handle));

                self.messages.clear();
                ConsoleService::clear();

                self.messages.push(Model::current_time_with_suffix("Interval started!"));
                true
            }
            Msg::Cancel => {
                self.job = None;
                self.messages.push(Model::current_time_with_suffix("Cancelled!"));
                ConsoleService::warn("Canceled");
                true
            }
            Msg::Done => {
                self.job = None;
                self.messages.push(Model::current_time_with_suffix("Done!"));

                ConsoleService::group();
                ConsoleService::info("Done!");
                ConsoleService::time_named_end("Timer");
                ConsoleService::group_end();
                true
            }
            Msg::Tick => {
                self.time = Model::current_time();
                ConsoleService::count_named("Tick");
                true
            }
            Msg::UpdateTime => {
                self.time = Model::current_time();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="wrapper">
                <div class="contents">
                    <div class="buttons">
                        <button onclick=self.link.callback(|_| Msg::StartTimeout)> { "Start Timeout" } </button>
                        <button onclick=self.link.callback(|_| Msg::StartInterval)>{ "Start Interval" }</button>
                        <button onclick=self.link.callback(|_| Msg::Cancel)>{ "Cancel" }</button>
                    </div>
                    <div class="time">
                        { &self.time }
                    </div>
                    <div class="messages">
                        { for self.messages.iter().rev().map(|message| html! { <p>{ message }</p> }) }
                    </div>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
