use std::ops::Deref;

use leptos::prelude::*;

const ASCII_ART: &str = r#"
            __      __                 
           |  \    |  \                
 _______  _| $$_   | $$____   _______  
|       \|   $$ \  | $$    \ |       \ 
| $$$$$$$\\$$$$$$  | $$$$$$$\| $$$$$$$\
| $$  | $$ | $$ __ | $$  | $$| $$  | $$
| $$  | $$ | $$|  \| $$  | $$| $$  | $$
| $$  | $$  \$$  $$| $$  | $$| $$  | $$
 \$$   \$$   \$$$$  \$$   \$$ \$$   \$$
"#;

#[component]
pub fn Banner(#[prop(into)] visible: Signal<IsVisible>) -> impl IntoView {
    // use a closure to make accessing the visible signal reactive
    move || {
        visible.read().then(|| {
            view! {
                <div data-testid="banner">
                    // hide the ascii art from screen readers as it's just a decorative element
                    <p class="mb-2 whitespace-pre text-primary" aria-hidden="true">
                        {ASCII_ART}
                    </p>
                    <p>"version 0.1.2"</p>
                    <p>
                        "type "<code class="text-green-theme">help</code>
                        " for a list of available commands"
                    </p>
                    <p>
                        "type "<code class="text-green-theme">help</code>
                        " [command] for help about a specific command"
                    </p>
                    <p>"type "<code class="text-green-theme">fetch</code> " to display summary"</p>
                </div>
            }
        })
    }
}

/// A boolean indicator of the visibility of the banner.
/// It's a newtype wrapper around a boolean to make it
/// unambiguous when providing and using the value as context.
#[derive(Clone, Copy)]
pub struct IsVisible(pub bool);

impl Deref for IsVisible {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn create_banner_toggle() -> (ReadSignal<IsVisible>, WriteSignal<IsVisible>) {
    let visible = RwSignal::new(IsVisible(true));
    provide_context(visible);
    visible.split()
}

pub fn use_banner_toggle() -> Option<(ReadSignal<IsVisible>, WriteSignal<IsVisible>)> {
    use_context::<RwSignal<IsVisible>>().map(|v| v.split())
}
