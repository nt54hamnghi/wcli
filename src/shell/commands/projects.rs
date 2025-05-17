use gloo_net::http::Request;
use icondata as i;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};

use super::Command;

const BASE_URL: &str = "https://api.github.com/users/nt54hamnghi/repos";

pub struct Projects;

impl Command for Projects {
    const NAME: &'static str = "projects";
    const DESCRIPTION: &'static str = "explore my projects";
    const USAGE: &'static str = "\t\
    projects";

    fn run(_: Vec<String>) -> Option<impl IntoView> {
        let repos = LocalResource::new(fetch_repo);
        Some(view! {
            <div class="grid gap-x-6 grid-cols-[max-content_max-content_auto]">
                <Transition fallback=move || {
                    view! { <p>"One moment..."</p> }
                }>
                    <span class="contents">
                        <span class="text-info">NAME</span>
                        <span class="text-info">DESCRIPTION</span>
                        <span class="text-info">STARS</span>
                    </span>
                    {move || Suspend::new(async move {
                        let repos = repos.await;
                        view! { {repos.into_iter().map(|r| r.into_view()).collect_view()} }
                    })}
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
                </Transition>
            </div>
        })
    }
}

#[component]
fn ProjectItem(
    #[prop(into)] name: String,
    #[prop(into)] desc: String,
    #[prop(optional, into)] url: String,
    #[prop(optional)] star: usize,
    #[prop(optional)] in_progress: bool,
) -> impl IntoView {
    view! {
        {if in_progress {
            Either::Left(
                view! {
                    <span class="contents">
                        <span>{name}</span>
                        <span>{desc}</span>
                        <span>"In Progress"</span>
                    </span>
                },
            )
        } else {
            Either::Right(
                view! {
                    <a class="contents group" href=url target="_blank" rel="noopener noreferrer">
                        <span class="group-hover:underline">{name}</span>
                        <span class="group-hover:underline">{desc}</span>
                        <span class="flex gap-1 items-center group-hover:underline">
                            <Icon icon=i::FaStarRegular height="1.125em" width="1.125em" />
                            <span>{star}</span>
                        </span>
                    </a>
                },
            )
        }}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Repository {
    name: String,
    html_url: String,
    description: String,
    stargazers_count: usize,
}

impl Repository {
    fn into_view(self) -> impl IntoView {
        let Self {
            name,
            html_url,
            description,
            stargazers_count,
        } = self;
        view! {
            <ProjectItem
                name=name
                desc=description
                url=html_url
                star=stargazers_count
                in_progress=false
            />
        }
    }
}

async fn fetch_repo() -> Vec<Repository> {
    // add yrc and sev proactively, which are private now
    // but will be published later
    let my_repos = ["seaq", "sublist3r-rs", "sev", "yrc"];
    let resp = Request::get(BASE_URL).send().await;

    let Ok(resp) = resp else {
        return Vec::new();
    };

    resp.json::<Vec<Repository>>()
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|r| my_repos.contains(&r.name.as_str()))
        .collect()
}
