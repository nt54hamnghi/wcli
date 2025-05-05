use leptos::{either::Either, prelude::*};

pub fn dispatch(input: String) -> impl IntoView {
    if input.is_empty() {
        return "".into_any();
    }

    let (cmd, args) = match input.split_once(' ') {
        Some((cmd, args)) => (cmd, args.split(' ').collect::<Vec<_>>()),
        None => (input.as_str(), vec![]),
    };

    match cmd {
        "echo" => echo(&args).into_any(),
        _ => not_found(cmd).into_any(),
    }
}

fn echo(args: &[&str]) -> impl IntoView {
    if args.is_empty() {
        Either::Left(view! { <br /> })
    } else {
        Either::Right(view! { <div class="text-white">{args.join(" ")}</div> })
    }
}

fn not_found(cmd: &str) -> impl IntoView {
    view! {
        <div class="text-white">
            <p>"command not found: "<span class="text-red-400">{cmd.to_owned()}</span></p>
            <p>
                "try "<span class="text-green-400">help</span>
                " to get a list of available commands"
            </p>
        </div>
    }
}
