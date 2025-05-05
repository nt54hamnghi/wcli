use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

// Modules
mod components;
mod pages;
mod shell;
mod stores;

// Top-Level pages
use pages::home::Home;
use pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}
