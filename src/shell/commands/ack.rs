use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;

use crate::shell::Command;

pub struct Ack;

impl Command for Ack {
    const NAME: &'static str = "ack";
    const DESCRIPTION: &'static str = "see acknowledgements";
    const USAGE: &'static str = "\t\
    ack";

    fn run(_: Vec<String>, _: SignalSetter<bool>) -> Option<impl IntoView> {
        Some(view! {
            <div>
                <p>"Special thanks to:"</p>
                <ul class="pl-4 list-disc">
                    <li>
                        <span class="font-semibold text-primary">"Niklas Ziermann"</span>
                        " - for making "
                        <a
                            class="underline"
                            href="https://www.youtube.com/watch?v=KCcU15nvFbI"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "this video"
                        </a>
                        " that helped me get started"
                    </li>
                    <li>
                        <span class="font-semibold text-primary">"Wensen (Vincent) Wu"</span>
                        " - for making "
                        <a
                            class="underline"
                            href="https://github.com/Cveinnt/LiveTerm"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "LiveTerm"
                        </a>
                        " that inspired this project"
                    </li>
                </ul>
            </div>
        })
    }
}
