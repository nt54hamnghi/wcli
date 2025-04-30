use leptos::{ev::Targeted, prelude::*};
use web_sys::{HtmlInputElement, KeyboardEvent};

#[component]
pub(super) fn Input(
    on_enter: impl FnMut(Targeted<KeyboardEvent, HtmlInputElement>) + 'static,
) -> impl IntoView {
    let (input, set_input) = signal(String::new());

    view! {
        <input
            type="text"
            value=input
            on:input:target=move |e| {
                set_input.set(e.target().value());
            }
            on:keydown:target=on_enter
        />
    }
}
