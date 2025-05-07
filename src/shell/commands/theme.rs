use std::str::FromStr;

use crate::theme::Theme as ThemeChoice;
use crate::theme::use_theme;

use super::Command;
use leptos::prelude::*;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy)]
pub struct Theme;

impl Command for Theme {
    const NAME: &'static str = "theme";
    const DESCRIPTION: &'static str = "change the theme";
    const USAGE: &'static str = r#"
    theme - pick a random theme
    theme [THEME] - use the specified theme"#;

    fn run(args: Vec<String>) -> Option<impl IntoView> {
        let (_theme, set_theme) = use_theme().unwrap();

        let selected = if args.is_empty() {
            ThemeChoice::random()
        } else {
            let value = args.first().expect("has at least 1 item");

            match ThemeChoice::from_str(value.as_str()) {
                Ok(t) => t,
                Err(_) => {
                    return Some(
                        view! {
                            <div class="text-fail">
                                <p>{format!("theme '{value}' is not supported")}</p>
                                <p>
                                    "available themes: "
                                    {ThemeChoice::iter()
                                        .map(|t| t.to_string())
                                        .collect::<Vec<_>>()
                                        .join(", ")}
                                </p>
                            </div>
                        }
                        .into_any(),
                    );
                }
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
}
