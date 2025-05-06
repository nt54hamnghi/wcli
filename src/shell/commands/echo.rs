use leptos::either::Either;
use leptos::prelude::*;

use super::Command;

pub struct Echo;

impl Command for Echo {
    const NAME: &'static str = "echo";
    const DESCRIPTION: &'static str = r"display a line of text";
    const USAGE: &'static str = r#"
    echo [STRING]"#;

    fn run(args: Vec<String>) -> Option<impl IntoView> {
        let result = if args.is_empty() {
            Either::Left(view! { <br /> })
        } else {
            Either::Right(view! { <div class="text-white">{args.join(" ")}</div> })
        };

        Some(result)
    }
}
