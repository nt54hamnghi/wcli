use leptos::prelude::*;

#[component]
pub fn Prompt(
    #[prop(default = "guess")] user: &'static str,
    #[prop(default = "host")] host: &'static str,
    #[prop(default = ":~$")] prefix: &'static str,
    #[prop(optional)] value: String,
) -> impl IntoView {
    view! {
        <div class="flex gap-4 items-center">
            <div class="inline-block text-white whitespace-nowrap">
                <span>{user}</span>
                <span class="text-red-400">@</span>
                <span>{host}</span>
                <span class="text-green-400">{prefix}</span>
            </div>
            {(!value.is_empty())
                .then(|| view! { <span class="flex-1 text-white whitespace-pre">{value}</span> })}
        </div>
    }
}
