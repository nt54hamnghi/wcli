use leptos::html;
use leptos::prelude::*;
use web_sys::js_sys;

use super::input::Input;

#[derive(Debug, Clone)]
struct Action {
    timestamp: u64,
    input: String,
    output: String,
}

impl Action {
    fn new(output: String) -> Self {
        let timestamp = (js_sys::Date::now() / 1000.0).round() as u64;
        Self {
            timestamp,
            input: output,
            output: "command not found".to_owned(),
        }
    }
}

#[component]
pub fn Interface() -> impl IntoView {
    let (history, set_history) = signal(Vec::<Action>::new());
    let (input, set_input) = signal("".to_owned());

    let div_ref: NodeRef<html::Div> = NodeRef::new();
    let scroll_right = move || {
        let div = div_ref.get().expect("should be mounted");
        div.scroll_to_with_x_and_y(div.scroll_width() as f64, 0.0);
    };
    let scroll_left = move || {
        let div = div_ref.get().expect("should be mounted");
        div.scroll_to_with_x_and_y(0.0, 0.0);
    };

    view! {
        <div class="flex overflow-auto flex-col gap-6 p-4 h-screen bg-gray-900" node_ref=div_ref>
            <For each=move || history.get() key=move |action| action.timestamp let(action)>
                {
                    view! {
                        <div>
                            <div class="flex gap-4 items-center">
                                <Prompt />
                                <p class="flex-1 text-white whitespace-pre">{action.input}</p>
                            </div>
                            <p class="text-white">{action.output}</p>
                        </div>
                    }
                }
            </For>
            <div class="flex gap-4 items-center">
                <Prompt />
                <Input
                    value=input
                    // FIXME: this would scroll right even when
                    // the change happens at the start of input
                    on_input=move |e| {
                        set_input.set(e.target().value());
                        scroll_right();
                    }
                    on_keydown=move |e| {
                        match e.key().as_str() {
                            "Enter" => {
                                set_history.write().push(Action::new(input.get()));
                                set_input.write().clear();
                            }
                            "ArrowLeft" => {}
                            "ArrowRight" => {}
                            "Home" => scroll_left(),
                            "End" => scroll_right(),
                            _ => {}
                        };
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn Prompt(
    #[prop(default = "guess")] user: &'static str,
    #[prop(default = "host")] host: &'static str,
    #[prop(default = ":~$")] prefix: &'static str,
) -> impl IntoView {
    view! {
        <div class="inline-block text-white whitespace-nowrap">
            <span>{user}</span>
            <span class="text-red-400">@</span>
            <span>{host}</span>
            <span class="text-green-400">{prefix}</span>
        </div>
    }
}
