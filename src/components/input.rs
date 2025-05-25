use std::time::Duration;

use leptos::ev::Targeted;
use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

use crate::shell::Palette;

const INPUT_ID: &str = "sole-input";

pub fn get_input_element() -> Option<HtmlInputElement> {
    let e = document()
        .get_element_by_id(INPUT_ID)?
        .unchecked_into::<HtmlInputElement>();
    Some(e)
}

/// Terminal-style input component with a custom blinking block cursor and scroll support
#[component]
pub(super) fn Input(
    /// Current input value
    value: ReadSignal<String>,
    /// Typeahead value used for auto-completion
    #[prop(into)]
    typeahead: Signal<String>,
    /// Reference to a container to support auto scroll when input overflows.
    /// The container must have `overflow-x: auto`.
    scroll_ref: NodeRef<html::Div>,
    /// The input event handler
    on_input: impl Fn(Targeted<Event, HtmlInputElement>) + 'static,
    /// The keydown event handler
    on_keydown: impl Fn(Targeted<KeyboardEvent, HtmlInputElement>) + 'static,
) -> impl IntoView {
    // position of the cursor in the input
    let (position, set_position) = signal(0);
    // whether the cursor is blinking
    let (is_blinking, set_is_blinking) = signal(false);
    // split the input into before and after the cursor
    let (before, after) = split_at(value, position);

    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let span_ref_before: NodeRef<html::Span> = NodeRef::new();

    let scroll_right = move || {
        let div = scroll_ref.get().expect("should be mounted");
        let scroll_diff = div.scroll_width() - div.client_width();
        // only scroll if the content is overflowing
        // and if the scroll position is not already at the right edge
        if scroll_diff > 0 && scroll_diff != div.scroll_left() {
            // to delay scrolling to after the browser's default auto-scroll to bring input into view
            set_timeout(
                move || div.set_scroll_left(scroll_diff),
                Duration::from_millis(10),
            );
        }
    };

    let scroll_left = move || {
        let div = scroll_ref.get().expect("should be mounted");
        // only scroll if not already at the left edge
        if div.scroll_left() > 0 {
            set_timeout(move || div.set_scroll_left(0), Duration::from_millis(10));
        }
    };

    let scroll_into_view = move || {
        let div = scroll_ref.get().expect("should be mounted");
        let span = span_ref_before.get().expect("should be mounted");
        // TODO: only scroll if the content is overflowings
        set_timeout(
            move || {
                // TODO: calculate the prompt offset_width dynamically
                // 225 is the offset_width of the prompt, which is hardcoded
                let scroll_diff = span.offset_width() - div.client_width() + 225;
                div.set_scroll_left(scroll_diff)
            },
            Duration::from_millis(5),
        );
    };

    Effect::new(move || {
        if value.read().is_empty() {
            set_position.set(0);
            scroll_left();
        }
    });

    view! {
        <div class="flex relative items-center">
            <div
                class="relative pr-12 text-base whitespace-pre"
                on:click=move |_| {
                    input_ref.get().expect("should be mounted").focus().unwrap();
                }
            >
                <span node_ref=span_ref_before>
                    {
                        let before = split_first(before);
                        view! {
                            <span class=move || {
                                if Palette::contains(before.read().0.as_str()) {
                                    "text-pass"
                                } else {
                                    "text-fail"
                                }
                            }>{move || before.read().0.clone()}</span>
                            <span>{move || before.read().1.clone()}</span>
                        }
                    }
                </span>
                // top-1/2 moves the top-left corner down to the middle of the parent's height
                // -translate-y-1/2 moves the element up by haft of its height
                <span
                    class=move || {
                        format!(
                            "inline-block absolute top-1/2 bg-base -translate-y-1/2 h-[1.125em] {}",
                            if is_blinking.get() { "animate-blink" } else { "" },
                        )
                    }
                    data-testid="cursor"
                >
                    " "
                </span>
                <span>{after}<span class="text-base opacity-60">{typeahead}</span></span>
            </div>
            <input
                type="text"
                id=INPUT_ID
                class="sr-only"
                autofocus
                node_ref=input_ref
                prop:value=value
                on:focus=move |_| set_is_blinking.set(true)
                on:blur=move |_| set_is_blinking.set(false)
                on:input:target=move |e| {
                    let diff = {
                        let new = e.target().value();
                        (new.len() as isize) - (value.read().len() as isize)
                    };
                    on_input(e);
                    scroll_into_view();
                    set_position.update(|p| *p = p.saturating_add_signed(diff));
                }
                // on_keydown might change the input value,
                // so any position updates should be done after on_keydown
                on:keydown:target=move |e| {
                    let kbe = e.clone();
                    on_keydown(e);
                    {
                        let len = value.read().len();
                        match kbe.key().as_str() {
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
                            "ArrowUp" | "ArrowDown" | "Tab" => {
                                set_position.set(len);
                                scroll_into_view();
                            }
                            _ => {}
                        }
                    }
                }
            />
        </div>
    }
}

/// Returns a derived signal that splits a string at a specified position
/// and return the parts before and after that position as derived signals.
fn split_at(s: ReadSignal<String>, mid: ReadSignal<usize>) -> (Signal<String>, Signal<String>) {
    let before = Signal::derive(move || s.get().split_at(mid.get()).0.to_owned());
    let after = Signal::derive(move || s.get().split_at(mid.get()).1.to_owned());

    (before, after)
}

/// Returns a derived signal that splits a string at the first space character, preserving the space in the second part.
/// If no space is found, the signal returns a tuple of the input string and an empty string.
fn split_first(s: Signal<String>) -> Signal<(String, String)> {
    Signal::derive(move || {
        s.read()
            .split_once(' ')
            .map(|(f, s)| (f.to_owned(), format!(" {s}")))
            .unwrap_or_else(|| (s.get(), "".to_owned()))
    })
}
