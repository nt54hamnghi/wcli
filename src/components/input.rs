use leptos::html;
use leptos::prelude::*;

#[component]
pub(super) fn Input(on_enter: impl Fn(ReadSignal<String>) + 'static) -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (position, set_position) = signal(0);
    let (before, after) = split_at(input, position);

    let input_element: NodeRef<html::Input> = NodeRef::new();

    view! {
        <div>
            <span
                class="text-white"
                on:click=move |_| {
                    input_element.get().expect("should be mounted").focus().unwrap();
                }
            >
                <span>{before}</span>
                <span class="text-center bg-white">"_"</span>
                <span>{after}</span>
            </span>
            <input
                type="text"
                class="sr-only"
                autofocus
                node_ref=input_element
                prop:value=input
                on:input:target=move |e| {
                    set_input.set(e.target().value());
                    set_position.set(input.read().len());
                }
                on:keydown:target=move |e| {
                    if e.key() == "Enter" {
                        on_enter(input);
                        set_input.write().clear();
                        set_position.set(0);
                    }
                }
            />
        </div>
    }
}

fn split_at(s: ReadSignal<String>, mid: ReadSignal<usize>) -> (Signal<String>, Signal<String>) {
    let before = Signal::derive(move || s.get().split_at(mid.get()).0.to_owned());
    let after = Signal::derive(move || s.get().split_at(mid.get()).1.to_owned());

    (before, after)
}
