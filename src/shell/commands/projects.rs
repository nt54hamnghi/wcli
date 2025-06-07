use std::collections::HashMap;
use std::sync::Arc;

use gloo_net::http::{Request, Response};
use icondata as i;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};
use web_sys::AbortSignal;

use super::{Command, UnexpectedOption};
use crate::config::{CONFIG, InProgress};

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
        let linguist = LocalResource::new(fetch_linguist);

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
                        let linguist = linguist.await?;
                        provide_context(Arc::new(linguist));
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
        <table
            class="hidden relative right-8 whitespace-nowrap border-separate table-auto lg:table border-spacing-x-8"
            data-testid="projects-table"
        >
            <thead>
                <tr class="text-left text-info">
                    <th class="font-normal" role="columnheader">
                        NAME
                    </th>
                    <th class="font-normal" role="columnheader">
                        DESCRIPTION
                    </th>
                    <th class="font-normal" role="columnheader">
                        LANGUAGE
                    </th>
                    <th class="font-normal" role="columnheader">
                        STARS
                    </th>
                    <th class="font-normal" role="columnheader">
                        STATUS
                    </th>
                </tr>
            </thead>
            <tbody>{items.clone().into_iter().map(|r| r.into_row_view()).collect_view()}</tbody>
        </table>

        <div class="flex flex-col gap-2 sm:gap-4 lg:hidden" data-testid="projects-list">
            {items.clone().into_iter().map(|r| r.into_card_view()).collect_view()}
        </div>
    }
}

#[component]
fn ProjectRow(
    name: String,
    desc: Option<String>,
    lang: Option<String>,
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
                        <td class="whitespace-normal max-w-[100ch]">{desc}</td>
                        <td>{lang.map(|l| view! { <Language lang=l /> })}</td>
                        <td></td>
                        <td class="italic opacity-90">Coming Soon</td>
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
                            <td class="whitespace-normal group-hover:underline max-w-[100ch]">
                                {desc}
                            </td>
                            <td>{lang.map(|l| view! { <Language lang=l /> })}</td>
                            <td class="group-hover:underline">
                                <Stargazers count=star />
                            </td>
                            <td class="group-hover:underline">Released</td>
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
    desc: Option<String>,
    lang: Option<String>,
    #[prop(optional)] url: String,
    #[prop(optional)] star: usize,
    #[prop(optional)] in_progress: bool,
) -> impl IntoView {
    view! {
        {if in_progress {
            Either::Left(
                view! {
                    <span class="flex flex-col gap-1 sm:gap-0">
                        <span class="text-info">{name}</span>
                        <span>{desc}</span>
                        {lang.map(|l| view! { <Language lang=l /> })}
                        <td class="italic opacity-90">Coming Soon</td>
                    </span>
                },
            )
        } else {
            Either::Right(
                view! {
                    <a
                        class="flex flex-col gap-1 sm:gap-0 group"
                        href=url
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <span class="group-hover:underline text-info">{name}</span>
                        <span>{desc}</span>
                        <span class="flex gap-2 items-center">
                            {lang.map(|l| view! { <Language lang=l /> })} <Stargazers count=star />
                        </span>
                    </a>
                },
            )
        }}
    }
}

#[component]
fn Stargazers(#[prop(optional)] count: usize) -> impl IntoView {
    view! {
        <span class="flex gap-1 items-center">
            <span class="inline-block relative bottom-[2px]">
                <Icon icon=i::FaStarRegular height="1rem" width="1rem" />
            </span>
            <span>{count}</span>
        </span>
    }
}

#[component]
fn Language(lang: String) -> impl IntoView {
    let linguist = expect_context::<Arc<Linguist>>();
    let color = linguist
        .get(&lang)
        .and_then(|l| l.color.as_deref())
        .unwrap_or("var(--color-white)");

    view! {
        <span class="flex gap-2 items-center">
            <span
                class="inline-block relative w-2 h-2 rounded-full bottom-[1px]"
                aria-hidden="true"
                style=format!("background-color: {}", color)
            ></span>
            <span>{lang}</span>
        </span>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Repository {
    name: String,
    description: Option<String>,
    html_url: Option<String>,
    stargazers_count: Option<usize>,
    language: Option<String>,
    #[serde(skip_deserializing)]
    in_progress: bool,
}

impl Repository {
    fn into_row_view(self) -> impl IntoView {
        view! {
            <ProjectRow
                name=self.name
                desc=self.description
                lang=self.language
                url=self.html_url.unwrap_or_default()
                star=self.stargazers_count.unwrap_or_default()
                in_progress=self.in_progress
            />
        }
    }

    fn into_card_view(self) -> impl IntoView {
        view! {
            <ProjectCard
                name=self.name
                desc=self.description
                lang=self.language
                url=self.html_url.unwrap_or_default()
                star=self.stargazers_count.unwrap_or_default()
                in_progress=self.in_progress
            />
        }
    }
}

impl From<InProgress> for Repository {
    fn from(value: InProgress) -> Self {
        Self {
            name: value.name,
            description: value.description,
            html_url: None,
            stargazers_count: None,
            language: value.language,
            in_progress: true,
        }
    }
}

async fn fetch_repos() -> Result<Vec<Repository>, Error> {
    let config = &CONFIG.github;

    let mut repos = get(&config.api_url())
        .await?
        .json::<Vec<Repository>>()
        .await?;

    if !config.repos.is_empty() {
        repos.retain(|r| config.repos.contains(&r.name));
    }

    // add in progress projects manually
    for item in config.in_progress.clone() {
        repos.push(item.into());
    }

    Ok(repos)
}

/// A linguist is a map of language names to their corresponding color.
/// See also: https://github.com/github-linguist/linguist
type Linguist = HashMap<String, Language>;

#[derive(Debug, Clone, Deserialize)]
struct Language {
    color: Option<String>,
}

async fn fetch_linguist() -> Result<Linguist, Error> {
    let url = "https://raw.githubusercontent.com/github/linguist/master/lib/linguist/languages.yml";

    let text = get(url).await?.text().await?;
    let linguist = serde_yaml::from_str(&text)?;

    Ok(linguist)
}

/// Make a GET request with a 5000ms timeout
async fn get(url: &str) -> Result<Response, Error> {
    let timeout_signal = AbortSignal::timeout_with_u32(5000);
    let resp = Request::get(url)
        .abort_signal(Some(&timeout_signal))
        .send()
        .await?;

    Ok(resp)
}
