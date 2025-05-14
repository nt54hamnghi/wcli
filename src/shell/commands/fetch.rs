use leptos::prelude::*;

use super::Command;

pub struct Fetch;

const ASCII_LOGO: &str = r#"
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⢿⣿⣿⢛⠹⣿⣿⡿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠿⠿⠏⡜⡔⡍⣡⢪⡪⡌⡩⡢⡣⠹⠿⠿⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢘⢜⢜⢜⢜⢜⢜⢜⢜⢜⢜⢜⡜⡜⡕⡕⣕⠅⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠡⡢⡲⡸⡸⡪⡪⡣⣓⢝⢜⢜⢎⢎⢇⢇⢇⢧⢣⢣⢳⢰⠤⡉⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⡿⠇⡕⡕⣕⢕⢕⡕⣕⢕⢕⢕⡕⣕⢕⢕⡕⣕⢕⢕⡕⣕⢕⢭⠸⢿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⡅⢆⢇⢇⢏⢎⢎⢕⢕⢕⢕⢕⡕⡕⡕⡕⣕⢕⢕⢕⢕⢕⢕⢕⢕⢕⠦⢨⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⡿⢂⢇⢗⢕⢕⢕⢝⢜⢜⢎⢕⢕⢕⢕⢝⢔⢵⢱⢕⢝⢜⢜⢎⢇⠧⡑⢿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⠯⠐⡇⡇⡇⠧⠓⡝⡜⠈⠀⠀⠑⡇⡏⡎⡇⠇⠁⠀⠑⢕⢕⠕⠕⠕⡝⡜⡂⢻⣿⣿⣿⣿⣿
⣿⣿⣿⠟⡡⡪⡱⢀⠥⡲⡑⡡⡣⡣⠀⠀⠀⠀⡱⡱⡱⡑⠀⠀⠀⠀⢸⡸⢄⠫⡱⣄⠡⠪⡢⢌⠻⣿⣿⣿
⣿⣿⠏⡸⡨⢊⠠⡪⡣⡃⡢⡣⠃⢇⠄⠀⠀⢀⢎⢎⢎⢧⠀⠀⠀⢀⠕⡊⡣⣃⠪⡲⡱⡁⡑⢕⢅⢹⣿⣿
⣿⡿⠨⡊⣴⡯⡸⡸⡸⡠⡡⡰⡕⠨⡪⡲⡸⡸⡸⡸⡱⡱⡱⡱⡢⡣⡃⡒⡆⣑⢨⢪⢪⢢⢽⣆⢑⡂⢿⣿
⣿⣇⣈⣾⣿⠃⢣⢣⢫⢪⢪⢪⠪⣨⣜⣘⠸⠸⠜⠎⠎⠮⠪⣊⣊⣪⡄⢕⢝⢜⢜⢜⢜⠜⠘⣿⣧⣈⣼⣿
⣿⣿⣿⣿⣿⠨⠄⠣⡣⡣⡳⠑⣱⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣌⠪⢪⢪⡪⠃⢅⠃⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡌⢊⣷⣔⢰⢰⢱⡈⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⢡⢪⢢⠢⣰⣿⢐⢡⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣾⣿⣿⣷⣬⡊⠎⠦⡙⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢋⢔⠕⣑⣥⣾⣿⣿⣷⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
"#;

impl Command for Fetch {
    const NAME: &'static str = "fetch";
    const DESCRIPTION: &'static str = "fetch info about me";
    const USAGE: &'static str = r#"
    fetch"#;

    fn run(_: Vec<String>) -> Option<impl IntoView> {
        Some(view! {
            <div class="flex gap-12 items-center">
                <FetchLogo />
                <FetchDetails />
            </div>
        })
    }
}

#[component]
fn FetchLogo() -> impl IntoView {
    view! { <p class="whitespace-pre text-orange-theme">{ASCII_LOGO}</p> }
}

#[component]
fn FetchDetails() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-0">
            <p>------------------------------------</p>
            <p class="flex gap-2 items-center">
                <span>"name: "</span>
                <span class="text-orange-theme">"Nghi Nguyen"</span>
            </p>
            <p class="flex gap-2 items-center">
                <span>"email: "</span>
                <a
                    href="mailto:hamnghi.nguyentrieu@gmail.com"
                    target="_blank"
                    class="text-orange-theme"
                >
                    "hamnghi.nguyentrieu@gmail.com"
                </a>
            </p>
            <p class="flex gap-2 items-center">
                <span>"github: "</span>
                <a
                    href="https://github.com/nt54hamnghi"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-orange-theme"
                >
                    "github.com/nt54hamnghi"
                </a>
            </p>
            <p class="flex gap-2 items-center">
                <span>"linkedin: "</span>
                <a
                    href="https://linkedin.com/in/hamnghi"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-orange-theme"
                >
                    "linkedin.com/in/hamnghi"
                </a>
            </p>
            <p>------------------------------------</p>
        </div>
    }
}
