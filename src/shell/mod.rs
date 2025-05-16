use std::str::FromStr;

pub use commands::clear::Clear;
pub use commands::echo::Echo;
pub use commands::fetch::Fetch;
pub use commands::help::Help;
pub use commands::projects::Projects;
pub use commands::theme::Theme;
pub use commands::{Command, Palette};
use leptos::prelude::*;

pub mod commands;

pub fn dispatch(input: String) -> impl IntoView {
    let input = input.trim();

    if input.is_empty() {
        return "".into_any();
    }

    let (cmd, args) = match input.split_once(' ') {
        Some((cmd, args)) => (
            cmd.to_owned(),
            args.split(' ').map(|s| s.to_owned()).collect::<Vec<_>>(),
        ),
        None => (input.to_owned(), vec![]),
    };

    match Palette::from_str(&cmd) {
        Ok(cmd) => match cmd {
            Palette::Echo => Echo::run(args).into_any(),
            Palette::Help => Help::run(args).into_any(),
            Palette::Theme => Theme::run(args).into_any(),
            Palette::Fetch => Fetch::run(args).into_any(),
            Palette::Clear => Clear::run(args).into_any(),
            Palette::Projects => Projects::run(args).into_any(),
        },
        Err(_) => not_found(cmd).into_any(),
    }
}

fn not_found(cmd: String) -> impl IntoView {
    view! {
        <div class="text-base">
            <p>"command not found: "<span class="text-fail">{cmd}</span></p>
            <p>"try "<span class="text-pass">help</span> " to get a list of available commands"</p>
        </div>
    }
}
