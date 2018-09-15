#[macro_use]
extern crate yew;

mod errors;
mod logtopus_port;

use logtopus_port::LogtopusPort;
use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct Context {
    pub console: ConsoleService,
    pub logtopus: LogtopusPort,
}

impl Context {
    pub fn new() -> Context {
        Context {
            console: ConsoleService::new(),
            logtopus: LogtopusPort::new(),
        }
    }
}

pub struct Model {
    value: String,
    results: Vec<String>,
}

impl Model {
    fn new() -> Model {
        Model {
            value: "".into(),
            results: Vec::new(),
        }
    }

    fn view_result(&self, r: &String) -> Html<Context, Model> {
        html! {
            <li>{ r }</li>
        }
    }
}

pub enum Msg {
    SetSearch(String),
    Search,
    Nope,
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model::new()
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetSearch(new_value) => {
                self.value = new_value;
                context.console.log(&format!("Value: {}", self.value));
                true
            }

            Msg::Search => {
                // Update your model on events
                context.console.log(&format!("Search for {}!", &self.value));
                let results = context.logtopus.search(&self.value).unwrap();
                for r in &results {
                    context.console.log(&format!("Result: {}", &r));
                }
                self.results = results;
                true
            }

            Msg::Nope => false,
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            // Render your model here
            <div class="view",>
                <input  type="text",
                        name="search",
                        value=&self.value,
                        oninput=|e| { Msg::SetSearch(e.value) },
                        onkeypress=|e| {
                            if e.key() == "Enter" { Msg::Search } else { Msg::Nope }
                        },/>
            </div>
            <div class="results",>
                <p>{"Results"}</p>
                <ul class="results",>
                    { for self.results.iter().map(|r| self.view_result(r)) }
                </ul>
            </div>
        }
    }
}
