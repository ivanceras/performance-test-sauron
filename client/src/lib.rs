#![deny(warnings)]
use sauron::dom::Measurements;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Msg {
    RunStart(Vec<Data>),
    Selected(usize),
}

// App and all its members should be Serializable by serde
#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    data: Option<Vec<Data>>,
    selected: Option<usize>,
    time_start: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    id: usize,
    label: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            data: None,
            selected: None,
            time_start: 0.0,
        }
    }
}

impl App {
    #[cfg(not(feature = "alt-syntax"))]
    fn view_data(&self, data: &[Data]) -> Node<Msg> {
        node! {
            <div class="data">
                {for d in data{
                    let id = d.id;
                    let label = d.label.to_string();
                    node!{
                        <div class="row">
                            <div class="col-md-12 test-data">
                                <span class=if let Some(selected) = self.selected {
                                        if selected == id {
                                            "selected"
                                        } else {
                                            ""
                                        }
                                    } else {
                                        ""
                                    }
                                    on_click=move|_e| Msg::Selected(id)
                                >
                                    {text(label)}
                                </span>
                            </div>
                        </div>
                    }
                }}
            </div>
        }
    }

    #[cfg(feature = "alt-syntax")]
    fn view_data(&self, data: &[Data]) -> Node<Msg> {
        div(
            [class("data")],
            [div(
                [class("row")],
                data.iter().map(|d| {
                    let id = d.id;
                    let label = d.label.to_string();
                    div(
                        [class("col-md-12 test-data")],
                        [span(
                            [
                                class(if let Some(selected) = self.selected {
                                    if selected == id {
                                        "selected"
                                    } else {
                                        ""
                                    }
                                } else {
                                    ""
                                }),
                                on_click(move |_e| Msg::Selected(id)),
                            ],
                            [text(label)],
                        )],
                    )
                }),
            )],
        )
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            let run_node = sauron::document()
                .get_element_by_id("run-sauron")
                .expect("node must exist");

            let closure: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
                let json_data = build_data();
                let data: Vec<Data> =
                    serde_json::from_str::<Vec<Data>>(&json_data).expect("must unserialize data");

                program.dispatch(Msg::RunStart(data));
            }));

            run_node
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .expect("Must register event listener");
            closure.forget();
        })
    }

    fn view(&self) -> Node<Msg> {
        if let Some(data) = &self.data {
            self.view_data(data)
        } else {
            node! { <span></span> }
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::RunStart(data) => {
                log::debug!("Start running..");
                self.selected = None;
                self.data = Some(data);
                self.time_start = sauron::now();
                Cmd::none().measure_with_name("run")
            }
            Msg::Selected(id) => {
                self.selected = Some(id);
                Cmd::none().measure_with_name("selected")
            }
        }
    }

    fn measurements(&self, measurements: Measurements) -> Cmd<Self, Msg> {
        let time_spent = sauron::now() - self.time_start;
        log::info!("total time spent: {}ms", time_spent.round());
        log::info!("Measurements here: {:#?}", measurements);
        let run_node = sauron::document()
            .get_element_by_id("run-sauron")
            .expect("node must exist");
        run_node.set_text_content(Some(&format!("{} ms", measurements.total_time.round())));
        Cmd::none().no_render()
    }

    fn style(&self) -> String {
        String::new()
    }
}

#[wasm_bindgen(module = "/build_data.js")]
extern "C" {
    fn build_data() -> String;
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let container = sauron::document()
        .get_element_by_id("sauron")
        .expect("node must exist");

    Program::append_to_mount(App::default(), &container);
}
