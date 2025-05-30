use std::str::FromStr;

use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use strum::{IntoEnumIterator, VariantNames};

use super::{Command, Palette};

#[derive(Debug, Clone, Copy)]
pub struct Help;

impl Command for Help {
    const NAME: &'static str = "help";
    const DESCRIPTION: &'static str = "show help message";
    const USAGE: &'static str = "\t\
    help            show the overview help
    help [command]  show help for a specific command";

    fn run(args: Vec<String>, _: SignalSetter<bool>) -> Option<impl IntoView> {
        let result = if args.is_empty() {
            let msg = Palette::iter().map(|c| c.one_line()).collect_view();
            view! {
                <div class="flex flex-col gap-4">
                    <div data-testid="help-commands">
                        <p>"Commands:"</p>
                        <div class="grid gap-x-6 grid-cols-[max-content_auto]">{msg}</div>
                    </div>
                    <div data-testid="help-keybindings">
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
                Ok(cmd) => {
                    view! { <div data-testid="help-command-each">{cmd.help()}</div> }.into_any()
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
        <span class="contents" data-testid="help-keybinding-item">
            <span class="pl-8 text-info">{key}</span>
            <span>{desc}</span>
        </span>
    }
}
