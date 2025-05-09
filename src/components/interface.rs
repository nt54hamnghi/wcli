use leptos::html;
use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::components::input::get_input_element;
use crate::stores::history::{create_history, Entry};

use super::history::History;
use super::input::Input;
use super::prompt::Prompt;

#[component]
pub fn Interface() -> impl IntoView {
    let (history, set_history) = create_history();
    let (input, set_input) = signal("".to_owned());
    let div_ref: NodeRef<html::Div> = NodeRef::new();

    let focus = move |_: MouseEvent| {
        get_input_element().focus().expect("should be focusable");
    };

    let blur = move |_: MouseEvent| {
        get_input_element().blur().expect("should be focusable");
    };

    Effect::new(move || {
        if !history.read().is_empty() {
            let div = div_ref.get().expect("should be mounted");
            div.scroll_to_with_x_and_y(0.0, div.scroll_height() as f64);
        }
    });

    view! {
        <div
            class="flex overflow-auto flex-col gap-6 p-4 pb-12 h-screen text-base border-3 bg-surface box-border border-unfocus focus-within:border-primary"
            node_ref=div_ref
            on:mouseup=focus
            on:mouseenter=focus
            on:mouseleave=blur
        >
            <History />
            <div class="flex gap-4 items-center">
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
                                set_input.write().clear();
                            }
                            "ArrowLeft" => {}
                            "ArrowRight" => {}
                            _ => {}
                        };
                    }
                />
            </div>
        </div>
    }
}
