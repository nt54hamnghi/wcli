use components::interface::Interface;
use config::CONFIG;
use leptos::prelude::*;
use leptos_meta::Title;
use stores::theme::create_theme;

// Modules
mod components;
mod config;
mod shell;
mod stores;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    view! {
        // sets the document title
        <Title text=CONFIG.title.clone() />
        <Home />
    }
}

/// Default Home Page
#[component]
fn Home() -> impl IntoView {
    let (theme, _set_theme) = create_theme();

    view! {
        <ErrorBoundary fallback=|_| {
            view! {
                <div class="p-4 w-screen h-screen text-base bg-surface">
                    <h1>"Something went wrong!"</h1>
                </div>
            }
        }>
            <Interface {..} id=move || { format!("theme-{}", theme.get()) } />
        </ErrorBoundary>
    }
}
