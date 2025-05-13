use leptos::prelude::*;
use strum::{Display, EnumIter, EnumString};

pub mod echo;
pub mod help;
pub mod theme;

#[derive(Debug, Clone, Copy, EnumString, EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Palette {
    Echo,
    Help,
    Theme,
}

pub trait Command {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    const USAGE: &'static str;

    fn run(args: Vec<String>) -> Option<impl IntoView>;

    fn help() -> impl IntoView {
        view! {
            <div class="text-base">
                <p>
                    <span class="text-green-theme">{Self::NAME}</span>
                    <span>" - " {Self::DESCRIPTION}</span>
                </p>
                <p class="mt-4">"Usage:"</p>
                <pre>{Self::USAGE}</pre>
            </div>
        }
    }

    fn suggest() -> Vec<String> {
        vec![]
    }
}
