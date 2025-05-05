use leptos::prelude::*;

use crate::shell::dispatch;
use crate::stores::history::use_history;

use super::prompt::Prompt;

#[component]
pub fn History() -> impl IntoView {
    let (history, _set_history) = use_history().expect("not yet created");
    view! {
        <For each=move || history.get() key=move |entry| entry.id() let(entry)>
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
