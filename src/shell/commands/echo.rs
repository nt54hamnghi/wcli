use leptos::either::Either;
use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;

use super::Command;

pub struct Echo;

impl Command for Echo {
    const NAME: &'static str = "echo";
    const DESCRIPTION: &'static str = "display a line of text";
    const USAGE: &'static str = "\t\
    echo [string]";

    fn run(args: Vec<String>, _: SignalSetter<bool>) -> Option<impl IntoView> {
        let result = if args.is_empty() {
            Either::Left(view! { <br /> })
        } else {
            Either::Right(view! { <div class="text-base">{args.join(" ")}</div> })
        };

        Some(result)
    }
}
