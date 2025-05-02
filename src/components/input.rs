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
            <div
                class="relative text-white"
                on:click=move |_| {
                    input_element.get().expect("should be mounted").focus().unwrap();
                }
            >
                <span>{before}</span>
                // Vertically center this span using top-1/2 and -translate-y-1/2
                // top-1/2 moves the top-left corner down to the middle of the parent's height
                // -translate-y-1/2 moves the element up by haft of its height
                <span class="inline-block absolute top-1/2 text-center bg-white opacity-50 duration-100 -translate-y-1/2 animate-blink h-[1.125em]">
                    "."
                </span>
                <span>{after}</span>
            </div>
            <input
                type="text"
                class="sr-only"
                autofocus
                node_ref=input_element
                prop:value=input
                on:input:target=move |e| {
                    let value = e.target().value();
                    let diff = (value.len() as isize) - (input.read().len() as isize);
                    set_position.update(|p| *p = p.saturating_add_signed(diff));
                    set_input.set(value);
                }
                on:keydown:target=move |e| {
                    match e.key().as_str() {
                        "Enter" => {
                            on_enter(input);
                            set_input.write().clear();
                            set_position.set(0);
                        }
                        "ArrowLeft" => {
                            set_position.update(|p| { *p = p.saturating_sub(1) });
                        }
                        "ArrowRight" => {
                            set_position
                                .update(|p| { *p = p.saturating_add(1).min(input.read().len()) });
                        }
                        "Home" => {}
                        "End" => {}
                        _ => {}
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
