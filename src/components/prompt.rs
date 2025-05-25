use leptos::prelude::*;

#[component]
pub fn Prompt(
    #[prop(default = "guess")] user: &'static str,
    #[prop(default = "host")] host: &'static str,
    #[prop(default = ":~$")] prefix: &'static str,
    #[prop(optional)] value: String,
) -> impl IntoView {
    view! {
        <div class="flex gap-4 items-center text-base" role="group" aria-label="command prompt">
            <code class="inline-block whitespace-nowrap">
                {user}<span class="text-red-theme">@</span>{host}
                // hide the prefix from screen readers as it's just a decorative element
                <span class="text-green-theme" aria-hidden="true">
                    {prefix}
                </span>
            </code>
            {(!value.is_empty())
                .then(|| view! { <code class="flex-1 whitespace-pre">{value}</code> })}
        </div>
    }
}
