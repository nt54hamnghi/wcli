use leptos::prelude::*;

const ASCII_ART: &str = r#"
 _                                 _     _ 
| |                               | |   (_)
| |__   __ _ _ __ ___  _ __   __ _| |__  _ 
| '_ \ / _` | '_ ` _ \| '_ \ / _` | '_ \| |
| | | | (_| | | | | | | | | | (_| | | | | |
|_| |_|\__,_|_| |_| |_|_| |_|\__, |_| |_|_|
                              __/ |        
                             |___/         
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
