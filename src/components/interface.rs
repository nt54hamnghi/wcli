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
    view! {
        <div class="flex overflow-auto flex-col gap-6 p-4 h-screen bg-gray-900">
            <For each=move || history.get() key=move |action| action.timestamp let(action)>
                {
                    view! {
                        <div>
                            <div class="flex gap-4 items-center">
                                <Prompt />
                                <p class="flex-1 text-white">{action.input}</p>
                            </div>
                            <p class="text-white">{action.output}</p>
                        </div>
                    }
                }
            </For>
            <div class="flex gap-4 items-center">
                <Prompt />
                <Input
                    on_enter=move |v| set_history.write().push(Action::new(v.get_untracked()))
                    {..}
                    class="flex-1 text-white"
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
