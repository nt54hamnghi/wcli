use gloo_net::http::Request;
use icondata as i;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};
use web_sys::AbortSignal;

use super::{Command, UnexpectedOption};
use crate::config::CONFIG;

#[derive(Debug, Clone, Copy)]
enum Format {
    Table,
    Json,
}

pub struct Projects;

impl Command for Projects {
    const NAME: &'static str = "projects";
    const DESCRIPTION: &'static str = "explore my projects";
    const USAGE: &'static str = "\t\
    projects             use table format
    projects -j, --json  use JSON format";

    fn run(args: Vec<String>, set_pending: SignalSetter<bool>) -> Option<impl IntoView> {
        let repos = LocalResource::new(fetch_repos);

        let format = match args.first().map(|s| s.as_str()).unwrap_or("") {
            "-j" | "--json" => Format::Json,
            opt if opt.starts_with('-') => {
                return Some(view! { <UnexpectedOption opt=opt usage=Self::USAGE /> }.into_any());
            },
            _ => Format::Table,
        };

        Some(view! {
            <Transition fallback=move || view! { <p>"One moment..."</p> } set_pending=set_pending>
                <ErrorBoundary fallback=|_| {
                    view! {
                        <div class="text-fail">
                            <p>"error: failed to load project data"</p>
                            <p>"try again later"</p>
                        </div>
                    }
                }>
                    {Suspend::new(async move {
                        repos
                            .await
                            .and_then(|repos| {
                                Ok(
                                    match format {
                                        Format::Table => {
                                            view! { <ProjectTable items=repos /> }.into_any()
                                        }
                                        Format::Json => {
                                            let json = serde_json::to_string_pretty(&repos)?;

                                            view! {
                                                <pre class="mt-2" data-testid="projects-json">
                                                    {json}
                                                </pre>
                                            }
                                                .into_any()
                                        }
                                    },
                                )
                            })
                    })}
                </ErrorBoundary>
            </Transition>
        }.into_any())
    }

    fn suggest() -> Vec<String> {
        vec!["projects -j".to_owned(), "projects --json".to_owned()]
    }
}

#[component]
fn ProjectTable(items: Vec<Repository>) -> impl IntoView {
    view! {
        <table class="hidden relative right-8 whitespace-nowrap border-separate table-auto lg:table border-spacing-x-8">
            <thead>
                <tr class="text-left text-info">
                    <th class="font-normal" role="columnheader">
                        NAME
                    </th>
                    <th class="font-normal" role="columnheader">
                        DESCRIPTION
                    </th>
                    <th class="font-normal" role="columnheader">
                        STARS
                    </th>
                </tr>
            </thead>
            <tbody>{items.clone().into_iter().map(|r| r.into_row_view()).collect_view()}</tbody>
        </table>

        <div class="flex flex-col gap-2 sm:gap-4 lg:hidden">
            {items.clone().into_iter().map(|r| r.into_card_view()).collect_view()}
        </div>
    }
}

#[component]
fn ProjectRow(
    name: String,
    desc: String,
    #[prop(optional)] url: String,
    #[prop(optional)] star: usize,
    #[prop(optional)] in_progress: bool,
) -> impl IntoView {
    view! {
        {if in_progress {
            Either::Left(
                view! {
                    <tr>
                        <td>{name}</td>
                        <td>{desc}</td>
                        <td class="opacity-60">"In Progress"</td>
                    </tr>
                },
            )
        } else {
            Either::Right(
                view! {
                    <tr>
                        <a
                            class="contents group"
                            href=url
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <td class="group-hover:underline">{name}</td>
                            <td class="group-hover:underline">{desc}</td>
                            <td class="group-hover:underline">
                                <span class="flex gap-1 items-center">
                                    <Icon icon=i::FaStarRegular height="1.125em" width="1.125em" />
                                    <span>{star}</span>
                                </span>
                            </td>
                        </a>
                    </tr>
                },
            )
        }}
    }
}

#[component]
fn ProjectCard(
    name: String,
    desc: String,
    #[prop(optional)] url: String,
    #[prop(optional)] star: usize,
    #[prop(optional)] in_progress: bool,
) -> impl IntoView {
    view! {
        {if in_progress {
            Either::Left(
                view! {
                    <span class="flex flex-col">
                        <span class="text-info">{name}</span>
                        <span>{desc}</span>
                        <span class="opacity-60">"In Progress"</span>
                    </span>
                },
            )
        } else {
            Either::Right(
                view! {
                    <a
                        class="flex flex-col group"
                        href=url
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <span class="group-hover:underline text-info">{name}</span>
                        <span>{desc}</span>
                        <span class="flex gap-1 items-center">
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
    fn into_row_view(self) -> impl IntoView {
        let Self {
            name,
            html_url,
            description,
            stargazers_count,
            in_progress,
        } = self;
        view! {
            <ProjectRow
                name=name
                desc=description
                url=html_url.unwrap_or_default()
                star=stargazers_count.unwrap_or_default()
                in_progress=in_progress
            />
        }
    }

    fn into_card_view(self) -> impl IntoView {
        let Self {
            name,
            html_url,
            description,
            stargazers_count,
            in_progress,
        } = self;
        view! {
            <ProjectCard
                name=name
                desc=description
                url=html_url.unwrap_or_default()
                star=stargazers_count.unwrap_or_default()
                in_progress=in_progress
            />
        }
    }
}

async fn fetch_repos() -> Result<Vec<Repository>, Error> {
    let config = &CONFIG.github;

    let timeout_signal = AbortSignal::timeout_with_u32(5000);
    let response = Request::get(&config.api_url())
        .abort_signal(Some(&timeout_signal))
        .send()
        .await?;

    let mut repos = response.json::<Vec<Repository>>().await?;
    repos.retain(|r| config.repos.contains(&r.name));

    // add in progress projects manually
    for item in &config.in_progress {
        repos.push(Repository {
            name: item.name.clone(),
            html_url: None,
            description: item.description.clone(),
            stargazers_count: None,
            in_progress: true,
        });
    }

    Ok(repos)
}
