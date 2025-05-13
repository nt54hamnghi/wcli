use std::time::Duration;

use leptos::html;
use leptos::prelude::*;

use super::banner::Banner;
use super::history::History;
use super::input::{Input, get_input_element};
use super::prompt::Prompt;
use crate::stores::history::{Entry, create_history};

#[component]
pub fn Interface() -> impl IntoView {
    let (input, set_input) = signal("".to_owned());
    // node ref to auto scroll when input or history output overflows
    let div_ref: NodeRef<html::Div> = NodeRef::new();
    // history of input entries
    let (history, set_history) = create_history();
    // current index of history
    let (current, set_current) = signal(0);
    // typeahead value used for auto-completion
    let typeahead = Signal::derive(move || {
        let input = input.read();
        let history = history.read();
        let candidates = history.iter().map(|e| e.input.as_str()).collect();

        get_typeahead(candidates, input.as_str(), 3)
    });

    let focus = move || {
        get_input_element().focus().expect("should be focusable");
    };

    let blur = move || {
        get_input_element().blur().expect("should be focusable");
    };

    let scroll_bottom = move || {
        let div = div_ref.get().expect("should be mounted");
        let scroll_diff = div.scroll_height() - div.client_height();
        // only scroll if the content is overflowing
        // and if the scroll position is not already at the bottom
        if scroll_diff > 0 && scroll_diff != div.scroll_top() {
            // to delay scrolling to after the browser's default auto-scroll to bring input into view
            set_timeout(
                move || div.set_scroll_top(scroll_diff),
                Duration::from_millis(25),
            );
        }
    };

    Effect::new(move || {
        // access the input signal to force re-run on input change
        // scope it to drop the read guard from `.read()` as soon as possible
        {
            input.read();
        }
        scroll_bottom();
    });

    view! {
        <div
            class="flex overflow-auto flex-col gap-6 p-4 h-screen text-base transition-colors duration-100 ease-in border-3 bg-surface box-border border-unfocus scroll-smooth focus-within:border-primary"
            node_ref=div_ref
            // to make the div focusable and can receive keyboard events
            // without placing it in the tab order
            tabindex="-1"
            on:keydown=move |_| focus()
            on:mouseenter=move |_| focus()
            on:mouseleave=move |_| blur()
        >
            <Banner />
            <History />
            <div class="flex gap-4 items-center pb-8">
                <Prompt />
                <Input
                    value=input
                    typeahead=typeahead
                    scroll_ref=div_ref
                    on_input=move |e| {
                        set_input.set(e.target().value());
                    }
                    on_keydown=move |e| {
                        match e.key().as_str() {
                            "Enter" => {
                                set_history.write().push(Entry::new(input.get()));
                                set_current.set(history.read().len());
                                set_input.write().clear();
                            }
                            "ArrowUp" => {
                                e.prevent_default();
                                let (idx, value) = prev(current, history);
                                set_current.set(idx);
                                set_input.set(value);
                            }
                            "ArrowDown" => {
                                e.prevent_default();
                                let (idx, value) = next(current, history);
                                set_current.set(idx);
                                set_input.set(value);
                            }
                            "Tab" => {
                                e.prevent_default();
                                let typeahead = typeahead.get();
                                set_input.write().push_str(&typeahead);
                            }
                            _ => {}
                        };
                    }
                />
            </div>
        </div>
    }
}

fn get_typeahead(mut candidates: Vec<&str>, input: &str, limit: usize) -> String {
    if input.len() < limit || candidates.is_empty() {
        return "".to_owned();
    }

    candidates.retain(|s| s.starts_with(input));

    let completion = match candidates.len() {
        0 => return "".to_owned(),
        1 => candidates[0],
        // find the longest common prefix among all candidates
        // this is to provide incremental completion
        _ => candidates
            .into_iter()
            .reduce(|first, second| {
                if first == second {
                    first
                } else if first.starts_with(second) {
                    second
                } else if second.starts_with(first) {
                    first
                } else {
                    let diff_idx = first
                        .chars()
                        .zip(second.chars())
                        .position(|(f, s)| f != s)
                        .expect("first and second should be different");

                    if first.len() < second.len() {
                        &first[..diff_idx]
                    } else {
                        &second[..diff_idx]
                    }
                }
            })
            .expect("not empty"),
    };

    completion[input.len()..].to_owned()
}

fn prev(current: ReadSignal<usize>, history: ReadSignal<Vec<Entry>>) -> (usize, String) {
    let history = history.read();
    let current = current.get();
    let value = history
        .get(current)
        .map(|e| e.input.as_str())
        .unwrap_or_default();

    for idx in (0..current).rev() {
        let prev = history
            .get(idx)
            .map(|e| e.input.as_str())
            .unwrap_or_default();

        if prev != value {
            return (idx, prev.to_owned());
        }
    }

    (0, value.to_owned())
}

fn next(current: ReadSignal<usize>, history: ReadSignal<Vec<Entry>>) -> (usize, String) {
    let history = history.read();
    let current = current.get();
    let value = history
        .get(current)
        .map(|e| e.input.as_str())
        .unwrap_or_default();

    for idx in current + 1..=history.len() {
        let next = history
            .get(idx)
            .map(|e| e.input.as_str())
            .unwrap_or_default();

        if next != value {
            return (idx, next.to_owned());
        }
    }

    (history.len(), value.to_owned())
}
