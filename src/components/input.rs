use leptos::ev::Targeted;
use leptos::html;
use leptos::prelude::*;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;

#[component]
pub(super) fn Input(
    value: ReadSignal<String>,
    on_input: impl Fn(Targeted<Event, HtmlInputElement>) + 'static,
    on_keydown: impl Fn(Targeted<KeyboardEvent, HtmlInputElement>) + 'static,
) -> impl IntoView {
    let (position, set_position) = signal(0);
    let (before, after) = split_at(value, position);

    let input_ref: NodeRef<html::Input> = NodeRef::new();

    view! {
        <div class="pr-8">
            <div
                class="relative text-white whitespace-pre"
                on:click=move |_| {
                    input_ref.get().expect("should be mounted").focus().unwrap();
                }
            >
                <span>{before}</span>
                // top-1/2 moves the top-left corner down to the middle of the parent's height
                // -translate-y-1/2 moves the element up by haft of its height
                <span class="inline-block absolute top-1/2 bg-white -translate-y-1/2 h-[1.125em] animate-blink">
                    "."
                </span>
                <span>{after}</span>
            </div>
            <input
                type="text"
                class="sr-only"
                autofocus
                node_ref=input_ref
                prop:value=value
                on:input:target=move |e| {
                    {
                        let new = e.target().value();
                        let diff = (new.len() as isize) - (value.read().len() as isize);
                        set_position.update(|p| *p = p.saturating_add_signed(diff));
                    }
                    on_input(e);
                }
                on:keydown:target=move |e| {
                    {
                        let len = value.read().len();
                        match e.key().as_str() {
                            "Enter" => set_position.set(0),
                            "ArrowLeft" => set_position.update(|p| { *p = p.saturating_sub(1) }),
                            "ArrowRight" => {
                                set_position.update(|p| { *p = p.saturating_add(1).min(len) })
                            }
                            "Home" => set_position.set(0),
                            "End" => set_position.set(len),
                            _ => {}
                        }
                    }
                    on_keydown(e);
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
