use leptos::prelude::*;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, Clone, Copy, Default, EnumIter, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Theme {
    #[default]
    Catppuccin,
    Everforest,
    Kanagawa,
    Nord,
    Dracula,
    TokyoNight,
    Github,
    Houston,
}

impl Theme {
    pub fn random() -> Theme {
        fastrand::choice(Theme::iter()).expect("Theme enum is non-empty")
    }
}

/// Creates a signal of a theme and provides it as context to the component tree.
/// It should be called only once in the root component.
pub fn create_theme() -> (ReadSignal<Theme>, WriteSignal<Theme>) {
    let theme = RwSignal::new(Theme::default());
    provide_context(theme);
    theme.split()
}

/// Retrieves the theme from the component tree context.
/// Returns `None` if no theme has been created.
pub fn use_theme() -> Option<(ReadSignal<Theme>, WriteSignal<Theme>)> {
    use_context::<RwSignal<Theme>>().map(|v| v.split())
}
