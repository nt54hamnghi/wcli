use icondata as i;
use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use leptos_icons::Icon;

use super::Command;
use crate::config::{CONFIG, Config};

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
    const DESCRIPTION: &'static str = "get a summary about me";
    const USAGE: &'static str = "\t\
    fetch";

    fn run(_: Vec<String>, _: SignalSetter<bool>) -> Option<impl IntoView> {
        Some(view! {
            <div class="flex flex-col gap-6 items-start lg:flex-row lg:gap-12 lg:items-center">
                <FetchLogo />
                <FetchDetails />
            </div>
        })
    }
}

#[component]
fn FetchLogo() -> impl IntoView {
    view! {
        <p class="text-xs whitespace-pre text-orange-theme" aria-hidden="true">
            {ASCII_LOGO}
        </p>
    }
}

#[component]
fn FetchDetails() -> impl IntoView {
    let Config {
        name,
        email,
        github,
        linkedin,
        ..
    } = CONFIG.clone();

    view! {
        <div class="flex flex-col gap-0 py-4 border-y">
            <p class="flex gap-2 items-center">
                <Icon icon=i::FaIdCardRegular height="1.125em" width="1.125em" />
                <span>"name: "</span>
                <span class="text-orange-theme">{name}</span>
            </p>
            <p class="flex gap-2 items-center">
                <Icon icon=i::FaEnvelopeSolid height="1.125em" width="1.125em" />
                <span>"email: "</span>
                <a
                    href=format!("mailto:{}", email)
                    target="_blank"
                    class="hover:underline text-orange-theme"
                >
                    {email.clone()}
                </a>
            </p>
            <p class="flex gap-2 items-center">
                <Icon icon=i::FaGithubBrands height="1.125em" width="1.125em" />
                <span>"github: "</span>
                <a
                    href=github.url()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:underline text-orange-theme"
                >
                    {github.short_url()}
                </a>
            </p>
            <p class="flex gap-2 items-center">
                <Icon icon=i::FaLinkedinBrands height="1.125em" width="1.125em" />
                <span>"linkedin: "</span>
                <a
                    href=linkedin.url()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:underline text-orange-theme"
                >
                    {linkedin.short_url()}
                </a>
            </p>
        </div>
    }
}
