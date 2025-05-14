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
pub fn Banner() -> impl IntoView {
    view! {
        <div>
            <p class="mb-2 whitespace-pre text-primary">{ASCII_ART}</p>
            <p>"version 0.1.0"</p>
            <p>
                "type "<span class="text-green-theme">help</span>
                " for a list of available commands"
            </p>
            <p>"type "<span class="text-green-theme">fetch</span> " to display summary"</p>
        </div>
    }
}
