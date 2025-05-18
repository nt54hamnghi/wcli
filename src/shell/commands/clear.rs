use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;

use super::Command;
use crate::components::banner::use_banner_toggle;
use crate::stores::history::use_history;

pub struct Clear;

impl Command for Clear {
    const NAME: &'static str = "clear";
    const DESCRIPTION: &'static str = "clear screen";
    const USAGE: &'static str = "\t\
    clear";

    fn run(_: Vec<String>, _: SignalSetter<bool>) -> Option<impl IntoView> {
        let (_history, set_history) = use_history().expect("not yet created");
        set_history.write().clear();

        let (_visible, set_visible) = use_banner_toggle().expect("not yet created");
        set_visible.write().0 = false;

        // clear doesn't return anything
        None::<()>
    }
}
