use std::ops::Deref;
use std::sync::LazyLock;
use std::time::Duration;

use leptos::html;
use leptos::prelude::*;

use super::banner::{Banner, create_banner_toggle};
use super::history::History;
use super::input::{Input, get_input_element};
use super::prompt::Prompt;
use crate::shell::Palette;
use crate::stores::history::{History, create_history};

static PRE_HISTORY: LazyLock<Vec<String>> = LazyLock::new(Palette::suggest);

#[component]
pub fn Interface() -> impl IntoView {
    // toggle banner visibility
    let (visible, set_visible) = create_banner_toggle();
    // input value
    let (input, set_input) = signal("".to_owned());
    // node ref to auto scroll when input or history output overflows
    let div_ref: NodeRef<html::Div> = NodeRef::new();
    // history of input entries
    let (history, set_history) = create_history();
    // whether history is still loading
    let (pending, set_pending) = signal(false);
    // current index of history
    let (current, set_current) = signal(0);
    // typeahead value used for auto-completion
    let typeahead = Signal::derive(move || {
        let input = input.read();
        let history = history.read();

        let mut candidates = history
            .commands()
            .iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>();

        candidates.extend(PRE_HISTORY.iter().map(|s| s.as_str()));

        use_typeahead(candidates, input.as_str(), 2)
    });

    let focus = move || {
        if let Some(e) = get_input_element() {
            e.focus().expect("should be focusable");
        }
    };

    let blur = move || {
        if let Some(e) = get_input_element() {
            e.blur().expect("should be focusable");
        }
    };

    let scroll_bottom = move || {
        let div = div_ref.get().expect("should be mounted");
        let scroll_diff = div.scroll_height() - div.client_height();
        // only scroll if the content is overflowing
        // and if the scroll position is not already at the bottom
        if scroll_diff > 0 && scroll_diff != div.scroll_top() {
            // to delay scrolling to after the browser's default auto-scroll to bring input into view
            set_timeout(
                move || div.set_scroll_top(scroll_diff),
                Duration::from_millis(25),
            );
        }
    };

    // scroll to the bottom when input changes
    Effect::new(move || {
        // access the input signal to force re-run on input change
        // scope it to drop the read guard from `.read()` as soon as possible
        {
            input.read();
        }
        scroll_bottom();
    });

    // focus on the input and scroll to the bottom
    // when history is fully loaded
    Effect::new(move || {
        if !pending.get() {
            focus();
            scroll_bottom();
        }
    });

    view! {
        <div
            class="flex overflow-auto flex-col gap-6 p-4 h-screen text-base transition-colors duration-100 ease-in font-terminal border-3 bg-surface box-border border-unfocus scroll-smooth focus-within:border-primary"
            node_ref=div_ref
            // to make the div focusable and can receive keyboard events
            // without placing it in the tab order
            tabindex="-1"
            on:keydown=move |_| focus()
            on:mouseenter=move |_| focus()
            on:mouseleave=move |_| blur()
        >
            <Banner visible=visible />
            <History set_pending=set_pending />
            <div class="flex gap-4 items-center pb-8">
                {move || {
                    let show = !pending.get();
                    show.then(move || {
                        view! {
                            <Prompt />
                            <Input
                                value=input
                                typeahead=typeahead
                                scroll_ref=div_ref
                                on_input=move |e| {
                                    set_input.set(e.target().value());
                                }
                                on_keydown=move |e| {
                                    match e.key().as_str() {
                                        "Enter" => {
                                            set_history.write().push(input.get());
                                            set_current.set(history.read().commands().len());
                                            set_input.write().clear();
                                        }
                                        "ArrowUp" => {
                                            e.prevent_default();
                                            let (idx, value) = prev(
                                                current.get(),
                                                history.read().deref(),
                                            );
                                            set_current.set(idx);
                                            set_input.set(value);
                                        }
                                        "ArrowDown" => {
                                            e.prevent_default();
                                            let (idx, value) = next(
                                                current.get(),
                                                history.read().deref(),
                                            );
                                            set_current.set(idx);
                                            set_input.set(value);
                                        }
                                        "Tab" => {
                                            e.prevent_default();
                                            let typeahead = typeahead.get();
                                            set_input.write().push_str(&typeahead);
                                        }
                                        "c" if e.ctrl_key() => {
                                            e.prevent_default();
                                            set_input.write().clear();
                                        }
                                        "l" if e.ctrl_key() => {
                                            e.prevent_default();
                                            set_history.write().clear();
                                            set_visible.write().0 = false;
                                        }
                                        _ => {}
                                    };
                                }
                            />
                        }
                    })
                }}
            </div>
        </div>
    }
}

fn use_typeahead(mut candidates: Vec<&str>, input: &str, limit: usize) -> String {
    if input.len() < limit || candidates.is_empty() {
        return "".to_owned();
    }

    candidates.retain(|s| s.starts_with(input));

    let completion = match candidates.len() {
        0 => return "".to_owned(),
        1 => candidates[0],
        // find the longest common prefix among all candidates
        // this is to provide incremental completion
        _ => candidates
            .into_iter()
            .reduce(|first, second| {
                if first == second {
                    first
                } else if first.starts_with(second) {
                    second
                } else if second.starts_with(first) {
                    first
                } else {
                    let diff_idx = first
                        .chars()
                        .zip(second.chars())
                        .position(|(f, s)| f != s)
                        .expect("first and second should be different");

                    if first.len() < second.len() {
                        &first[..diff_idx]
                    } else {
                        &second[..diff_idx]
                    }
                }
            })
            .expect("not empty"),
    };

    completion[input.len()..].to_owned()
}

fn prev(current: usize, history: &History) -> (usize, String) {
    let idx = current.saturating_sub(1);
    let value = history.commands().get(idx).cloned().unwrap_or_default();

    (idx, value)
}

fn next(current: usize, history: &History) -> (usize, String) {
    let idx = current.saturating_add(1).min(history.commands().len());
    let value = history.commands().get(idx).cloned().unwrap_or_default();

    (idx, value)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn history() -> History {
        let mut history = History::new();
        history.push("clear".to_string());
        history.push("echo".to_string());
        history.push("fetch".to_string());
        history
    }

    #[rstest]
    #[case::normal(2, 1, "echo")]
    #[case::at_start(0, 0, "clear")]
    fn test_prev(
        history: History,
        #[case] current: usize,
        #[case] expected_idx: usize,
        #[case] expected_value: &str,
    ) {
        let (idx, value) = prev(current, &history);
        assert_eq!(idx, expected_idx);
        assert_eq!(value, expected_value);
    }

    #[rstest]
    #[case::normal(0, 1, "echo")]
    #[case::at_end(2, 3, "")]
    fn test_next(
        history: History,
        #[case] current: usize,
        #[case] expected_idx: usize,
        #[case] expected_value: &str,
    ) {
        let (idx, value) = next(current, &history);
        assert_eq!(idx, expected_idx);
        assert_eq!(value, expected_value);
    }

    #[test]
    fn test_empty_history() {
        let history = History::new();

        // Test prev with empty history
        let (idx, value) = prev(0, &history);
        assert_eq!(idx, 0);
        assert_eq!(value, "");

        // Test next with empty history
        let (idx, value) = next(0, &history);
        assert_eq!(idx, 0);
        assert_eq!(value, "");
    }

    #[fixture]
    fn candidates() -> Vec<&'static str> {
        vec!["clear", "echo", "fetch", "fetching", "fetched"]
    }

    #[test]
    fn test_use_typeahead_empty_candidates() {
        assert_eq!(use_typeahead(vec![], "test", 2), "");
    }

    #[rstest]
    #[case::below("e", 2, "")]
    #[case::at("ec", 2, "ho")]
    #[case::above("ech", 2, "o")]
    fn test_use_typeahead_with_limit(
        candidates: Vec<&str>,
        #[case] input: &str,
        #[case] limit: usize,
        #[case] expected: &str,
    ) {
        let result = use_typeahead(candidates, input, limit);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::no_matches("xyz", "")]
    // exact single match, nothing to complete.
    #[case::exact_match("clear", "")]
    // options are "fetch", "fetching", "fetched", so the longest common prefix is "fetch"
    // thus, after typing "fe", the completion should be "tch"
    #[case::common_prefix("fe", "tch")]
    fn test_use_typeahead_matches(
        candidates: Vec<&str>,
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let result = use_typeahead(candidates, input, 2);
        assert_eq!(result, expected);
    }
}
