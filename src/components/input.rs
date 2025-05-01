use leptos::prelude::*;

#[component]
pub(super) fn Input(on_enter: impl Fn(ReadSignal<String>) + 'static) -> impl IntoView {
    let (input, set_input) = signal(String::new());

    view! {
        <input
            type="text"
            value=input
            prop:value=input
            on:input:target=move |e| {
                set_input.set(e.target().value());
            }
            on:keydown:target=move |e| {
                if e.key() == "Enter" {
                    on_enter(input);
                    set_input.write().clear();
                }
            }
        />
    }
}
