use std::str::FromStr;

use leptos::prelude::*;
use strum::IntoEnumIterator;

use super::Command;
use crate::stores::theme::{Theme as ThemeChoice, use_theme};

#[derive(Debug, Clone, Copy)]
pub struct Theme;

impl Command for Theme {
    const NAME: &'static str = "theme";
    const DESCRIPTION: &'static str = "change the theme";
    const USAGE: &'static str = r#"
    theme           pick a random theme
    theme [THEME]   use the specified theme
    theme -l        list available themes"#;

    fn run(args: Vec<String>) -> Option<impl IntoView> {
        let (_theme, set_theme) = use_theme().unwrap();

        let selected = if args.is_empty() {
            ThemeChoice::random()
        } else {
            let value = args.first().expect("has at least 1 item");

            if value == "-l" || value == "--list" {
                return Some(ThemeList().into_any());
            };

            match ThemeChoice::from_str(value.as_str()) {
                Ok(t) => t,
                Err(_) => {
                    return Some(
                        view! {
                            <div class="text-fail">
                                <p>{format!("theme '{value}' is not supported")}</p>
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
            view! {
                <div class="text-base">
                    <p>{format!("theme '{selected}' selected")}</p>
                </div>
            }
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
    view! {
        <p>
            "available themes: "
            {ThemeChoice::iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ")}
        </p>
    }
}
