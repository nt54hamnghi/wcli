use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use strum::{Display, EnumIter, EnumString, VariantNames};

pub mod clear;
pub mod echo;
pub mod fetch;
pub mod help;
pub mod projects;
pub mod theme;

#[derive(Debug, Clone, Copy, EnumString, EnumIter, Display, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Palette {
    Clear,
    Echo,
    Fetch,
    Help,
    Projects,
    Theme,
}

impl Palette {
    pub fn contains(s: &str) -> bool {
        Palette::VARIANTS.contains(&s)
    }
}

pub trait Command {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    const USAGE: &'static str;

    fn run(args: Vec<String>, set_pending: SignalSetter<bool>) -> Option<impl IntoView>;

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

#[component]
fn UnexpectedOption(#[prop(into)] opt: String, #[prop(into)] usage: String) -> impl IntoView {
    let msg = format!("unexpected flag: {}", opt);
    view! {
        <p class="text-fail">{msg}</p>
        <p>
            <p class="mt-4">"Usage:"</p>
            <pre>{usage}</pre>
        </p>
    }
}
