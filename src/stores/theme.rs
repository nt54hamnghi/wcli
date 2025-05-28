use leptos::prelude::*;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator, VariantNames};

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, EnumIter, EnumString, Display, VariantNames,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Theme {
    #[default]
    Catppuccin,
    Dracula,
    Everforest,
    GithubDark,
    GithubLight,
    Houston,
    Kanagawa,
    Nord,
    Precious,
    RosePine,
    TokyoNight,
}

impl Theme {
    pub fn random_except(current: Theme) -> Theme {
        let themes = Theme::iter().filter(|t| *t != current).collect::<Vec<_>>();
        fastrand::choice(themes).expect("Theme enum is non-empty")
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
