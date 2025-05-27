use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;

use super::prompt::Prompt;
use crate::shell::dispatch;
use crate::stores::history::use_history;

#[component]
pub fn History(#[prop(into)] set_pending: SignalSetter<bool>) -> impl IntoView {
    let (history, _set_history) = use_history().expect("not yet created");
    view! {
        <For each=move || history.read().buffer().to_vec() key=move |entry| entry.id() let(entry)>
            {
                view! {
                    <article>
                        <Prompt value=entry.input.clone() />
                        {dispatch(entry.input, set_pending)}
                    </article>
                }
            }
        </For>
    }
}
