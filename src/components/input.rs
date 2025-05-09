use leptos::ev::Targeted;
use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

const INPUT_ID: &str = "sole-input";

pub fn get_input_element() -> HtmlInputElement {
    document()
        .get_element_by_id(INPUT_ID)
        .expect("<input /> with id should exist")
        .unchecked_into()
}

/// Terminal-style input component with block cursor and scroll support
#[component]
pub(super) fn Input(
    /// Current input value
    value: ReadSignal<String>,
    /// Reference to a container to support auto scroll when input overflows.
    /// The container must have `overflow-x: auto`.
    scroll_ref: NodeRef<html::Div>,
    /// The input event handler
    on_input: impl Fn(Targeted<Event, HtmlInputElement>) + 'static,
    /// The keydown event handler
    on_keydown: impl Fn(Targeted<KeyboardEvent, HtmlInputElement>) + 'static,
) -> impl IntoView {
    let (position, set_position) = signal(0);
    let (is_blinking, set_is_blinking) = signal(false);
    let (before, after) = split_at(value, position);

    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let span_ref_before: NodeRef<html::Span> = NodeRef::new();

    let scroll_right = move || {
        let div = scroll_ref.get().expect("should be mounted");
        div.scroll_to_with_x_and_y(div.scroll_width() as f64, 0.0);
    };

    let scroll_left = move || {
        let div = scroll_ref.get().expect("should be mounted");
        div.scroll_to_with_x_and_y(0.0, 0.0);
    };

    // scroll to the right when the input is updated and overflows
    let scroll_on_input = move || {
        let div = scroll_ref.get().expect("should be mounted");
        let span = span_ref_before.get().expect("should be mounted");

        // TODO: replace 200 with offset from prompt
        if span.offset_width() + 200 >= div.client_width() {
            div.scroll_to_with_x_and_y(div.scroll_width() as f64, 0.0);
        }
    };

    view! {
        <div class="flex items-center pr-12">
            <div
                class="relative text-base whitespace-pre"
                on:click=move |_| {
                    input_ref.get().expect("should be mounted").focus().unwrap();
                }
            >
                <span node_ref=span_ref_before>{before}</span>
                // top-1/2 moves the top-left corner down to the middle of the parent's height
                // -translate-y-1/2 moves the element up by haft of its height
                <span class=move || {
                    format!(
                        "inline-block absolute top-1/2 bg-base -translate-y-1/2 h-[1.125em] {}",
                        if is_blinking.get() { "animate-blink" } else { "" },
                    )
                }>" "</span>
                <span>{after}</span>
            </div>
            <input
                type="text"
                id=INPUT_ID
                class="scale-0"
                autofocus
                node_ref=input_ref
                prop:value=value
                on:focus=move |_| set_is_blinking.set(true)
                on:blur=move |_| set_is_blinking.set(false)
                on:input:target=move |e| {
                    {
                        let new = e.target().value();
                        let diff = (new.len() as isize) - (value.read().len() as isize);
                        set_position.update(|p| *p = p.saturating_add_signed(diff));
                    }
                    on_input(e);
                    scroll_on_input();
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
                            "Home" => {
                                set_position.set(0);
                                scroll_left();
                            }
                            "End" => {
                                set_position.set(len);
                                scroll_right();
                            }
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
