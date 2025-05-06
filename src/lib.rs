use components::interface::Interface;
use leptos::prelude::*;
use theme::create_theme;

// Modules
mod components;
mod shell;
mod stores;
mod theme;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    view! { <Home /> }
}

/// Default Home Page
#[component]
fn Home() -> impl IntoView {
    let (theme, _set_theme) = create_theme();

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="p-4 w-screen h-screen text-base bg-surface">
                    <h1>"Something went wrong!"</h1>

                    <p class="text-fail">"Errors: "</p>
                    // Render a list of errors as strings - good for development purposes
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        }}

                    </ul>
                </div>
            }
        }>
            <Interface {..} id=move || { format!("theme-{}", theme.get()) } />
        </ErrorBoundary>
    }
}
