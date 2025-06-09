use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use strum::{Display, EnumIter, EnumString, VariantNames};

use self::ack::Ack;
use self::clear::Clear;
use self::echo::Echo;
use self::fetch::Fetch;
use self::help::Help;
use self::projects::Projects;
use self::stack::Stack;
use self::theme::Theme;

pub mod ack;
pub mod clear;
pub mod echo;
pub mod fetch;
pub mod help;
pub mod projects;
pub mod stack;
pub mod theme;

#[derive(Debug, Clone, Copy, EnumString, EnumIter, Display, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Palette {
    Ack,
    Clear,
    Echo,
    Fetch,
    Help,
    Projects,
    Stack,
    Theme,
}

impl Palette {
    pub fn contains(s: &str) -> bool {
        Self::VARIANTS.contains(&s)
    }

    pub fn run(self, args: Vec<String>, set_pending: SignalSetter<bool>) -> impl IntoView {
        match self {
            Self::Ack => Ack::run(args, set_pending).into_any(),
            Self::Clear => Clear::run(args, set_pending).into_any(),
            Self::Echo => Echo::run(args, set_pending).into_any(),
            Self::Fetch => Fetch::run(args, set_pending).into_any(),
            Self::Help => Help::run(args, set_pending).into_any(),
            Self::Projects => Projects::run(args, set_pending).into_any(),
            Self::Stack => Stack::run(args, set_pending).into_any(),
            Self::Theme => Theme::run(args, set_pending).into_any(),
        }
    }

    /// Returns a help message
    pub fn help(self) -> impl IntoView {
        match self {
            Self::Ack => Ack::help().into_any(),
            Self::Clear => Clear::help().into_any(),
            Self::Echo => Echo::help().into_any(),
            Self::Fetch => Fetch::help().into_any(),
            Self::Help => Help::help().into_any(),
            Self::Projects => Projects::help().into_any(),
            Self::Stack => Stack::help().into_any(),
            Self::Theme => Theme::help().into_any(),
        }
    }

    /// Returns a one-line description
    pub fn one_line(self) -> impl IntoView {
        let (name, desc) = match self {
            Self::Ack => (Ack::NAME, Ack::DESCRIPTION),
            Self::Clear => (Clear::NAME, Clear::DESCRIPTION),
            Self::Echo => (Echo::NAME, Echo::DESCRIPTION),
            Self::Fetch => (Fetch::NAME, Fetch::DESCRIPTION),
            Self::Help => (Help::NAME, Help::DESCRIPTION),
            Self::Theme => (Theme::NAME, Theme::DESCRIPTION),
            Self::Projects => (Projects::NAME, Projects::DESCRIPTION),
            Self::Stack => (Stack::NAME, Stack::DESCRIPTION),
        };

        view! {
            <span class="pl-8 text-green-theme" data-testid="help-oneline">
                {name}
            </span>
            <span class="text-foreground">{desc}</span>
        }
    }

    pub fn suggest() -> Vec<String> {
        let mut h = Self::VARIANTS
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        h.extend(Theme::suggest());
        h.extend(Help::suggest());
        h.extend(Projects::suggest());
        h
    }
}

pub trait Command {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    const USAGE: &'static str;

    fn run(args: Vec<String>, set_pending: SignalSetter<bool>) -> Option<impl IntoView>;

    fn help() -> impl IntoView {
        view! {
            <div class="text-foreground">
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
