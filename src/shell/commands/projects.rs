use gloo_net::http::Request;
use icondata as i;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};
use web_sys::AbortSignal;

use super::{Command, UnexpectedOption};

const BASE_URL: &str = "https://api.github.com/users/nt54hamnghi/repos";

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
        <table class="relative right-8 whitespace-nowrap border-separate table-auto border-spacing-x-8">
            <thead>
                <tr class="text-left text-info">
                    <th role="columnheader">NAME</th>
                    <th role="columnheader">DESCRIPTION</th>
                    <th role="columnheader">STARS</th>
                </tr>
            </thead>
            <tbody>{items.into_iter().map(|r| r.into_view()).collect_view()}</tbody>
        </table>
    }
}

#[component]
fn ProjectRow(
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
                    <tr>
                        <td>{name}</td>
                        <td>{desc}</td>
                        <td>"In Progress"</td>
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
            <ProjectRow
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
    // proactively add 'sev' and 'yrc' to the list
    // these are private GitHub repos, but will be published later
    let names = ["seaq", "sublist3r-rs", "sev", "yrc"];
    let timeout_signal = AbortSignal::timeout_with_u32(5000);
    let response = Request::get(BASE_URL)
        .abort_signal(Some(&timeout_signal))
        .send()
        .await?;

    let mut repos = response.json::<Vec<Repository>>().await?;
    repos.retain(|r| names.contains(&r.name.as_str()));

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

    Ok(repos)
}
