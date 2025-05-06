use leptos::html;
use leptos::prelude::*;

use crate::stores::history::{Entry, create_history};

use super::history::History;
use super::input::Input;
use super::prompt::Prompt;

#[component]
pub fn Interface() -> impl IntoView {
    let (_history, set_history) = create_history();
    let (input, set_input) = signal("".to_owned());
    let div_ref: NodeRef<html::Div> = NodeRef::new();

    view! {
        <div class="flex overflow-auto flex-col gap-6 p-4 h-screen bg-surface" node_ref=div_ref>
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
