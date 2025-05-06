use leptos::prelude::*;

#[component]
pub fn Prompt(
    #[prop(default = "guess")] user: &'static str,
    #[prop(default = "host")] host: &'static str,
    #[prop(default = ":~$")] prefix: &'static str,
    #[prop(optional)] value: String,
) -> impl IntoView {
    view! {
        <div class="flex gap-4 items-center text-base">
            <div class="inline-block whitespace-nowrap">
                <span>{user}</span>
                <span class="text-red-theme">@</span>
                <span>{host}</span>
                <span class="text-green-theme">{prefix}</span>
            </div>
            {(!value.is_empty())
                .then(|| view! { <span class="flex-1 whitespace-pre">{value}</span> })}
        </div>
    }
}
