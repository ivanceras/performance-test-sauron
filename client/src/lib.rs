use sauron::dom::Measurements;
use sauron::js_sys::TypeError;
use sauron::prelude::*;
use sauron::web_sys::Response;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate log;

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
        }
    }
}

impl App {
    fn view_using_macro(&self, data: &[Data]) -> Node<Msg> {
        node! {
            <div>
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

    fn view_using_fn_calls(&self, data: &[Data]) -> Node<Msg> {
        div(
            vec![],
            vec![div(
                vec![class("row")],
                data.iter()
                    .map(|d| {
                        let id = d.id;
                        let label = d.label.to_string();
                        div(
                            vec![class("col-md-12 test-data")],
                            vec![span(
                                vec![
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
                                vec![text(label)],
                            )],
                        )
                    })
                    .collect::<Vec<_>>(),
            )],
        )
    }
}

impl Component<Msg> for App {
    fn init(&self) -> Cmd<Self, Msg> {
        use sauron::wasm_bindgen::JsCast;

        Cmd::new(|program| {
            let run_node = sauron::document()
                .get_element_by_id("run-sauron")
                .expect("node must exist");

            let closure: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
                let json_data = get_build_data();
                log::debug!("got new data: {}", json_data);
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
            self.view_using_macro(data)
            //self.view_using_fn_calls(data)
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
                Cmd::measure()
            }
            Msg::Selected(id) => {
                self.selected = Some(id);
                Cmd::none()
            }
        }
    }

    fn measurements(&mut self, measurements: Measurements) -> Cmd<Self, Msg> {
        log::info!("Measurements here: {:#?}", measurements);
        let mut cmd = Cmd::new(move |program| {
            let run_node = sauron::document()
                .get_element_by_id("run-sauron")
                .expect("node must exist");
            run_node.set_text_content(Some(&format!("{} ms", measurements.total_time.round())));
        });
        cmd.should_update_view = false;
        cmd
    }
}

#[wasm_bindgen(module = "/build_data.js")]
extern "C" {
    fn get_build_data() -> String;
}

/// The serialized_state is supplied by the generated page from the webserver.
/// The generated page in index function has a main function which is supplied by a json text
/// serialized state. This json text is deserialized and used here as our `App` value which
/// will then be injected into the view
#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let container = sauron::document()
        .get_element_by_id("sauron")
        .expect("node must exist");

    Program::new_append_to_mount(App::default(), &container);
}
