use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use leptos_icons::Icon;

use super::Command;
use crate::config::{CONFIG, StackItem};

pub struct Stack;

impl Command for Stack {
    const NAME: &'static str = "stack";
    const DESCRIPTION: &'static str = "view my tech stack";
    const USAGE: &'static str = "\t\
    stack";

    fn run(_: Vec<String>, _: SignalSetter<bool>) -> Option<impl IntoView> {
        let stack = CONFIG
            .stack
            .iter()
            .map(
                |(title, items)| view! { <StackSection title=title.clone() items=items.clone() /> },
            )
            .collect_view();

        Some(view! { <div class="flex flex-col gap-6">{stack}</div> })
    }
}

#[component]
fn StackSection(title: String, items: Vec<StackItem>) -> impl IntoView {
    view! {
        <div>
            <h3 class="pb-1 text-primary">{title.to_uppercase()}</h3>
            <div class="flex flex-wrap gap-4">
                {items.into_iter().map(|i| view! { <StackBadge item=i /> }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn StackBadge(item: StackItem) -> impl IntoView {
    let StackItem { name, icon, color } = item;
    let color = color.unwrap_or("var(--color-white)".to_owned());
    view! {
        <div
            class="flex gap-2 items-center py-2 px-3 rounded-md border text-foreground"
            style=format!("border-color: {color}")
        >
            <Icon icon=icon height="1.5rem" width="1.5rem" {..} style=format!("color: {color}") />
            <span class="flex-1">{name}</span>
        </div>
    }
}
