use leptos::html;
use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::components::input::get_input_element;
use crate::stores::history::{Entry, create_history};

use super::history::History;
use super::input::Input;
use super::prompt::Prompt;

#[component]
pub fn Interface() -> impl IntoView {
    let (_history, set_history) = create_history();
    let (input, set_input) = signal("".to_owned());
    let div_ref: NodeRef<html::Div> = NodeRef::new();

    let focus = move |_: MouseEvent| {
        get_input_element().focus().expect("should be focusable");
    };

    let blur = move |_: MouseEvent| {
        get_input_element().blur().expect("should be focusable");
    };

    view! {
        <div
            class="overflow-auto min-h-screen group border-y-2 border-x-3 bg-surface border-unfocus focus-within:border-primary"
            on:mouseup=focus
            on:mouseenter=focus
            on:mouseleave=blur
            node_ref=div_ref
        >
            <div class="h-6 group-focus-within:bg-primary" />
            <div class="flex flex-col gap-6 p-4">
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
        </div>
    }
}
