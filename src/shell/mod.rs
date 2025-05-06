use std::str::FromStr;

use commands::echo::Echo;
use commands::help::Help;
use commands::theme::Theme;
use commands::{Command, Palette};
use leptos::prelude::*;

mod commands;

pub fn dispatch(input: String) -> impl IntoView {
    if input.is_empty() {
        return "".into_any();
    }

    let (cmd, args) = match input.trim_start().split_once(' ') {
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
