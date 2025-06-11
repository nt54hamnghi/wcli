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
            .map(|(title, items)| view! { <StackSection title=title items=items /> })
            .collect_view();

        Some(view! { <div class="flex flex-col gap-6">{stack}</div> })
    }
}

#[component]
fn StackSection(title: &'static str, items: &'static [StackItem]) -> impl IntoView {
    view! {
        <div>
            <h3 class="flex gap-2 items-center pb-1">
                <span class="font-bold text-primary">">"</span>
                <span>{title.to_uppercase()}</span>
            </h3>
            <div class="flex flex-wrap gap-4">
                {items.iter().map(|i| view! { <StackBadge item=i /> }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn StackBadge(item: &'static StackItem) -> impl IntoView {
    let name = item.name.as_str();
    let icon = item.icon;
    let color = item.color.as_deref().unwrap_or("var(--color-white)");

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
