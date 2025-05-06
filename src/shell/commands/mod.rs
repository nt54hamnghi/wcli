use leptos::prelude::*;
use strum::{EnumIter, EnumString};

pub mod echo;
pub mod help;

#[derive(Debug, Clone, Copy, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Palette {
    Echo,
    Help,
}

pub trait Command {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    const USAGE: &'static str;

    fn run(args: Vec<String>) -> Option<impl IntoView>;

    fn help() -> impl IntoView {
        view! {
            <div class="text-white">
                <p>
                    <span class="text-green-400">{Self::NAME}</span>
                    <span>" - " {Self::DESCRIPTION}</span>
                </p>
                <p class="mt-4">"Usage:"</p>
                <pre>{Self::USAGE}</pre>
            </div>
        }
    }
}
