use std::str::FromStr;

use leptos::prelude::*;
use strum::{IntoEnumIterator, VariantNames};

use super::clear::Clear;
use super::echo::Echo;
use super::fetch::Fetch;
use super::theme::Theme;
use super::{Command, Palette};

impl Palette {
    /// Returns a one-line description
    fn one_line(self) -> impl IntoView {
        let (name, desc) = match self {
            Self::Echo => (Echo::NAME, Echo::DESCRIPTION),
            Self::Help => (Help::NAME, Help::DESCRIPTION),
            Self::Theme => (Theme::NAME, Theme::DESCRIPTION),
            Self::Fetch => (Fetch::NAME, Fetch::DESCRIPTION),
            Self::Clear => (Clear::NAME, Clear::DESCRIPTION),
        };

        view! {
            <span class="pl-4 text-green-theme">{name}</span>
            <span class="text-base">{desc}</span>
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Help;

impl Command for Help {
    const NAME: &'static str = "help";
    const DESCRIPTION: &'static str = "show help message";
    const USAGE: &'static str = r#"
    help            show the overview help
    help [COMMAND]  show help for a specific command"#;

    fn run(args: Vec<String>) -> Option<impl IntoView> {
        let result = if args.is_empty() {
            let msg = Palette::iter().map(|c| c.one_line()).collect_view();
            view! {
                <div class="flex flex-col gap-4">
                    <div>
                        <p>"Commands:"</p>
                        <div class="grid gap-x-6 grid-cols-[max-content_auto]">{msg}</div>
                    </div>
                    <div>
                        <p>"Keybindings:"</p>
                        <div class="grid gap-x-6 grid-cols-[max-content_auto]">
                            <Keybinding key="[arrow up]" desc="previous command" />
                            <Keybinding key="[arrow down]" desc="next command" />
                            <Keybinding key="[ctrl+c]" desc="clear input" />
                            <Keybinding key="[ctrl+l]" desc="clear screen" />
                            <Keybinding key="[tab]" desc="trigger completion" />
                        </div>
                    </div>
                </div>
            }
            .into_any()
        } else {
            let cmd = args.first().expect("has at least 1 item");

            match Palette::from_str(cmd.as_str()) {
                Ok(cmd) => match cmd {
                    Palette::Echo => Echo::help().into_any(),
                    Palette::Help => Help::help().into_any(),
                    Palette::Theme => Theme::help().into_any(),
                    Palette::Fetch => Fetch::help().into_any(),
                    Palette::Clear => Clear::help().into_any(),
                },
                Err(_) => {
                    return Some(
                        view! {
                            <div class="text-fail">
                                <p>{format!("command '{cmd}' is not supported")}</p>
                                <p>"available commands: " {Palette::VARIANTS.join(", ")}</p>
                            </div>
                        }
                        .into_any(),
                    );
                },
            }
        };

        Some(result)
    }

    fn suggest() -> Vec<String> {
        Palette::iter().map(|c| format!("help {}", c)).collect()
    }
}

#[component]
fn Keybinding(#[prop(into)] key: &'static str, #[prop(into)] desc: &'static str) -> impl IntoView {
    view! {
        <span class="pl-4 text-info">{key}</span>
        <span>{desc}</span>
    }
}
