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
                                set_current.update(|c| { *c = c.saturating_sub(1) });
                                let value = history
                                    .read()
                                    .get(current.get())
                                    .map(|e| e.input.clone())
                                    .unwrap_or_default();
                                set_input.set(value);
                            }
                            "ArrowDown" => {
                                e.prevent_default();
                                let value = history
                                    .with(|h| {
                                        set_current
                                            .update(|c| { *c = c.saturating_add(1).min(h.len()) });
                                        h.get(current.get())
                                            .map(|e| e.input.clone())
                                            .unwrap_or_default()
                                    });
                                set_input.set(value);
                            }
                            _ => {}
                        };
                    }
                />
            </div>
        </div>
    }
}
