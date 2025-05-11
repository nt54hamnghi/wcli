use std::str::FromStr;

use leptos::prelude::*;
use strum::IntoEnumIterator;

use super::echo::Echo;
use super::theme::Theme;
use super::{Command, Palette};

impl Palette {
    /// Returns a one-line description
    fn one_line(self) -> impl IntoView {
        let (name, desc) = match self {
            Self::Echo => (Echo::NAME, Echo::DESCRIPTION),
            Self::Help => (Help::NAME, Help::DESCRIPTION),
            Self::Theme => (Theme::NAME, Theme::DESCRIPTION),
        };

        view! {
            <div class="text-base">
                <span class="text-green-theme">{name}</span>
                <span>" - " {desc}</span>
            </div>
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Help;

impl Command for Help {
    const NAME: &'static str = "help";
    const DESCRIPTION: &'static str = "show help message";
    const USAGE: &'static str = r#"
    help - show the overview help
    help [COMMAND] - show help for a specific command"#;

    fn run(args: Vec<String>) -> Option<impl IntoView> {
        let result = if args.is_empty() {
            let msg = Palette::iter().map(|c| c.one_line()).collect_view();
            view! { <div class="flex flex-col gap-2">{msg}</div> }.into_any()
        } else {
            let cmd = args.first().expect("has at least 1 item");

            match Palette::from_str(cmd.as_str()) {
                Ok(cmd) => match cmd {
                    Palette::Echo => Echo::help().into_any(),
                    Palette::Help => Help::help().into_any(),
                    Palette::Theme => Theme::help().into_any(),
                },
                Err(_) => {
                    return Some(
                        view! {
                            <div class="text-fail">
                                <p>{format!("command '{cmd}' is not supported")}</p>
                                <p>
                                    "available commands: "
                                    {Palette::iter()
                                        .map(|t| t.to_string())
                                        .collect::<Vec<_>>()
                                        .join(", ")}
                                </p>
                            </div>
                        }
                        .into_any(),
                    );
                },
            }
        };

        Some(result)
    }
}
