use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="p-4 w-screen h-screen text-base bg-surface">
            <h1>"Uh oh!" <br /> "We couldn't find that page!"</h1>
        </div>
    }
}
