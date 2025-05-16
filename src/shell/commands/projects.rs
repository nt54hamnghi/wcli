use leptos::either::{Either, either};
use leptos::prelude::*;

use super::Command;

pub struct Projects;

impl Command for Projects {
    const NAME: &'static str = "projects";
    const DESCRIPTION: &'static str = "explore my projects";
    const USAGE: &'static str = r#"
    projects"#;

    fn run(_: Vec<String>) -> Option<impl IntoView> {
        Some(view! {
            <div class="grid gap-x-6 grid-cols-[max-content_max-content_auto]">
                <span class="text-info">NAME</span>
                <span class="text-info">DESCRIPTION</span>
                <span class="text-info">STARS</span>
                <ProjectItem
                    name="seaq"
                    desc="Fetch and process web content with your favorite LLMs and prompts"
                    star=10
                />
                <ProjectItem
                    name="sev"
                    desc="Securely inject environment variables with secrects"
                    in_progress=true
                />
                <ProjectItem
                    name="yrc"
                    desc="You Remember Correctly - A memorable password generator"
                    in_progress=true
                />
                <ProjectItem
                    name="sublist3r-rs"
                    desc="A simple and minimal Rust rewrite of Sublist3r"
                    star=2
                />
            </div>
        })
    }
}

#[component]
fn ProjectItem(
    #[prop(into)] name: String,
    #[prop(into)] desc: String,
    #[prop(optional)] star: usize,
    #[prop(optional)] in_progress: bool,
) -> impl IntoView {
    view! {
        <span>{name}</span>
        <span>{desc}</span>
        {if in_progress {
            Either::Left(view! { <span>"In Progress"</span> })
        } else {
            Either::Right(view! { <span>{star}</span> })
        }}
    }
}
