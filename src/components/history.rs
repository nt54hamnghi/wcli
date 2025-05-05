use leptos::prelude::*;
use web_sys::js_sys;

use crate::shell::dispatch;

use super::prompt::Prompt;

#[derive(Debug, Clone)]
pub struct Entry {
    pub timestamp: u64,
    pub input: String,
}

impl Entry {
    pub fn new(input: String) -> Self {
        let timestamp = (js_sys::Date::now() / 1000.0).round() as u64;
        Self { timestamp, input }
    }
}

#[component]
pub fn History(history: ReadSignal<Vec<Entry>>) -> impl IntoView {
    view! {
        <For each=move || history.get() key=move |entry| entry.timestamp let(entry)>
            {
                view! {
                    <div>
                        <Prompt value=entry.input.clone() />
                        {dispatch(entry.input)}
                    </div>
                }
            }
        </For>
    }
}
