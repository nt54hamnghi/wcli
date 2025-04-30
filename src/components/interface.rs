use leptos::prelude::*;

use super::input::Input;

#[component]
pub fn Interface() -> impl IntoView {
    let (output, set_output) = signal(String::new());
    view! {
        <div class="overflow-auto flex-col gap-4 p-2 h-screen bg-gray-800">
            <p class="text-white">{output}</p>
            <div class="flex gap-4 items-center">
                <Prompt host="hamnghi.com" />
                <Input
                    on_enter=move |e| {
                        if e.key() == "Enter" {
                            set_output.set(e.target().value());
                        }
                    }
                    {..}
                    class="flex-1 text-white"
                />
            </div>
        </div>
    }
}

#[component]
fn Prompt(
    #[prop(default = "guess")] user: &'static str,
    #[prop(default = "host")] host: &'static str,
    #[prop(default = ":~$")] prefix: &'static str,
) -> impl IntoView {
    view! {
        <div class="inline-block text-white whitespace-nowrap">
            <span>{user}</span>
            <span class="text-red-400">@</span>
            <span>{host}</span>
            <span class="text-green-400">{prefix}</span>
        </div>
    }
}
