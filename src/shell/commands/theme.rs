use std::str::FromStr;

use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use strum::{IntoEnumIterator, VariantNames};

use super::{Command, UnexpectedOption};
use crate::stores::theme::{Theme as ThemeChoice, use_theme};

#[derive(Debug, Clone, Copy)]
pub struct Theme;

impl Command for Theme {
    const NAME: &'static str = "theme";
    const DESCRIPTION: &'static str = "change the theme";
    const USAGE: &'static str = "\t\
    theme             pick a random theme
    theme [name]      use the specified theme
    theme -l, --list  list available themes";

    fn run(args: Vec<String>, _: SignalSetter<bool>) -> Option<impl IntoView> {
        let (theme, set_theme) = use_theme().unwrap();

        let selected = if args.is_empty() {
            let current = theme.get();
            ThemeChoice::random_except(current)
        } else {
            let opt = args.first().expect("has at least 1 item");

            if opt.starts_with('-') {
                match opt.as_str() {
                    "-l" | "--list" => return Some(view! { <ThemeList /> }.into_any()),
                    _ => {
                        return Some(
                            view! { <UnexpectedOption opt=opt usage=Self::USAGE /> }.into_any(),
                        );
                    },
                }
            };

            match ThemeChoice::from_str(opt.as_str()) {
                Ok(t) => t,
                Err(_) => {
                    return Some(
                        view! {
                            <div class="text-fail">
                                <p>{format!("theme '{opt}' is not supported")}</p>
                                <ThemeList />
                            </div>
                        }
                        .into_any(),
                    );
                },
            }
        };

        set_theme.set(selected);

        Some(
            view! { <p class="text-foreground">{format!("theme '{selected}' selected")}</p> }
                .into_any(),
        )
    }

    fn suggest() -> Vec<String> {
        let mut opts = vec!["theme -l".to_owned(), "theme --list".to_owned()];
        opts.extend(ThemeChoice::iter().map(|t| format!("theme {}", t)));
        opts
    }
}

#[component]
fn ThemeList() -> impl IntoView {
    view! { <p>"available themes: " {ThemeChoice::VARIANTS.join(", ")}</p> }
}
