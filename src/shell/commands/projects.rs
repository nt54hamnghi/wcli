use gloo_net::http::Request;
use icondata as i;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};

use super::Command;

const BASE_URL: &str = "https://api.github.com/users/nt54hamnghi/repos";

pub struct Projects;

impl Command for Projects {
    const NAME: &'static str = "projects";
    const DESCRIPTION: &'static str = "explore my projects";
    const USAGE: &'static str = "\t\
    projects             use table format
    projects -j, --json  use JSON format";

    fn run(args: Vec<String>, set_pending: SignalSetter<bool>) -> Option<impl IntoView> {
        let repos = LocalResource::new(fetch_repos);
        Some(view! {
            <Transition
                fallback=move || {
                    view! { <p>"One moment..."</p> }
                }
                set_pending=set_pending
            >
                {move || {
                    let first = args.first().cloned();
                    Suspend::new(async move {
                        let repos = repos.await;
                        if first.is_some_and(|opts| opts == "-j" || opts == "--json") {
                            Either::Left(
                                view! { <pre>{serde_json::to_string_pretty(&repos).unwrap()}</pre> },
                            )
                        } else {
                            Either::Right(view! { <ProjectTable items=repos /> })
                        }
                    })
                }}
            </Transition>
        })
    }

    fn suggest() -> Vec<String> {
        vec!["projects -j".to_owned(), "projects --json".to_owned()]
    }
}

#[component]
fn ProjectTable(items: Vec<Repository>) -> impl IntoView {
    view! {
        <div class="grid gap-x-6 grid-cols-[max-content_max-content_auto]">
            <span class="contents">
                <span class="text-info">NAME</span>
                <span class="text-info">DESCRIPTION</span>
                <span class="text-info">STARS</span>
            </span>
            {items.into_iter().map(|r| r.into_view()).collect_view()}
        </div>
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
    html_url: Option<String>,
    description: String,
    stargazers_count: Option<usize>,
    #[serde(skip_deserializing)]
    in_progress: bool,
}

impl Repository {
    fn into_view(self) -> impl IntoView {
        let Self {
            name,
            html_url,
            description,
            stargazers_count,
            in_progress,
        } = self;
        view! {
            <ProjectItem
                name=name
                desc=description
                url=html_url.unwrap_or_default()
                star=stargazers_count.unwrap_or_default()
                in_progress=in_progress
            />
        }
    }
}

async fn fetch_repos() -> Vec<Repository> {
    // proactively add 'sev' and 'yrc' to the list
    // these are private GitHub repos, but will be published later
    let my_repos = ["seaq", "sublist3r-rs", "sev", "yrc"];
    let resp = Request::get(BASE_URL).send().await;

    let Ok(resp) = resp else {
        return Vec::new();
    };

    let mut repos = resp
        .json::<Vec<Repository>>()
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|r| my_repos.contains(&r.name.as_str()))
        .collect::<Vec<_>>();

    // add in progress projects manually
    // these are private GitHub repos, but will be published later
    repos.extend([
        Repository {
            name: "sev".to_owned(),
            html_url: None,
            description: "Securely inject environment variables with secrets".to_owned(),
            stargazers_count: None,
            in_progress: true,
        },
        Repository {
            name: "yrc".to_owned(),
            html_url: None,
            description: "You Remember Correctly - A memorable password generator".to_owned(),
            stargazers_count: None,
            in_progress: true,
        },
    ]);

    repos
}
