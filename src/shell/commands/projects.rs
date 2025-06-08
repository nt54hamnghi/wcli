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
                        FORKS
                    </th>
                    <th class="font-normal" role="columnheader">
                        STATUS
                    </th>
                </tr>
            </thead>
            <tbody>
                {items.clone().into_iter().map(|r| view! { <ProjectRow repo=r /> }).collect_view()}
            </tbody>
        </table>

        <div class="flex flex-col gap-2 sm:gap-4 lg:hidden" data-testid="projects-list">
            {items.clone().into_iter().map(|r| view! { <ProjectCard repo=r /> }).collect_view()}
        </div>
    }
}

#[component]
fn ProjectRow(repo: Repository) -> impl IntoView {
    match repo {
        Repository::Public {
            released:
                Released {
                    name,
                    description,
                    html_url,
                    stargazers_count,
                    language,
                    forks,
                },
        } => Either::Right(view! {
            <tr>
                <a
                    class="contents group"
                    href=html_url
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    <td class="group-hover:underline">{name}</td>
                    <td class="whitespace-normal group-hover:underline max-w-[100ch]">
                        {description}
                    </td>
                    <td>{language.map(|l| view! { <Language lang=l /> })}</td>
                    <td class="group-hover:underline">
                        <Stargazers count=stargazers_count />
                    </td>
                    <td class="group-hover:underline">
                        <Forks count=forks />
                    </td>
                    <td class="group-hover:underline">Released</td>
                </a>
            </tr>
        }),
        Repository::Private {
            in_progress:
                InProgress {
                    name,
                    description,
                    language,
                },
        } => Either::Left(view! {
            <tr>
                <td>{name}</td>
                <td class="whitespace-normal max-w-[100ch]">{description}</td>
                <td>{language.map(|l| view! { <Language lang=l /> })}</td>
                // no stargazers_count
                <td></td>
                // no forks
                <td></td>
                <td class="italic opacity-90">Coming Soon</td>
            </tr>
        }),
    }
}

#[component]
fn ProjectCard(repo: Repository) -> impl IntoView {
    match repo {
        Repository::Public {
            released:
                Released {
                    name,
                    description,
                    html_url,
                    stargazers_count,
                    language,
                    forks,
                },
        } => Either::Left(view! {
            <a
                class="flex flex-col gap-1 sm:gap-0 group"
                href=html_url
                target="_blank"
                rel="noopener noreferrer"
            >
                <span class="group-hover:underline text-info">{name}</span>
                <span>{description}</span>
                <span class="flex gap-2 items-center">
                    {language.map(|l| view! { <Language lang=l /> })}
                    <Stargazers count=stargazers_count /> <Forks count=forks />
                </span>
            </a>
        }),
        Repository::Private {
            in_progress:
                InProgress {
                    name,
                    description,
                    language,
                },
        } => Either::Right(view! {
            <span class="flex flex-col gap-1 sm:gap-0">
                <span class="text-info">{name}</span>
                <span>{description}</span>
                {language.map(|l| view! { <Language lang=l /> })}
                <td class="italic opacity-90">Coming Soon</td>
            </span>
        }),
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
fn Forks(#[prop(optional)] count: usize) -> impl IntoView {
    view! {
        <span class="flex gap-1 items-center">
            <span class="inline-block relative bottom-[2px]">
                <Icon icon=i::FaCodeForkSolid height="1rem" width="1rem" />
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
#[serde(from = "Released", tag = "status", rename_all = "lowercase")]
enum Repository {
    Public {
        #[serde(flatten)]
        released: Released,
    },
    Private {
        #[serde(flatten)]
        in_progress: InProgress,
    },
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Released {
    name: String,
    description: Option<String>,
    html_url: String,
    stargazers_count: usize,
    language: Option<String>,
    forks: usize,
}

impl From<Released> for Repository {
    fn from(released: Released) -> Self {
        Repository::Public { released }
    }
}

async fn fetch_repos() -> Result<Vec<Repository>, Error> {
    let config = &CONFIG.github;

    let mut repos = get(&config.api_url())
        .await?
        .json::<Vec<Repository>>()
        .await?;

    if !config.repos.is_empty() {
        // only include repositories listed in the config
        repos.retain(|r| {
            match r {
                Repository::Public { released } => config.repos.contains(&released.name),
                // `#[serde(from = "Released")]` deserializes the response into `Released`
                // before converting to `Repository`, so `Private` variants are not possible
                Repository::Private { .. } => unreachable!(),
            }
        });
    }

    // add in progress projects manually
    for item in config.in_progress.clone() {
        repos.push(Repository::Private { in_progress: item });
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
