use leptos::prelude::{ReadSignal, RwSignal, WriteSignal, provide_context, use_context};
use web_sys::js_sys;

#[derive(Debug, Clone)]
pub struct Entry {
    timestamp: u64,
    pub input: String,
}

impl Entry {
    pub fn new(input: String) -> Self {
        let timestamp = (js_sys::Date::now() / 1000.0).round() as u64;
        Self { timestamp, input }
    }

    pub fn id(&self) -> u64 {
        self.timestamp
    }
}

type History = Vec<Entry>;

/// Creates a signal of a history store and provides it as context to the component tree.
/// It should be called only once in the root component.
pub fn create_history() -> (ReadSignal<History>, WriteSignal<History>) {
    let history = RwSignal::new(History::new());
    provide_context(history);
    history.split()
}

/// Retrieves the history store from the component tree context.
/// Returns `None` if no history store has been created.
pub fn use_history() -> Option<(ReadSignal<History>, WriteSignal<History>)> {
    use_context::<RwSignal<History>>().map(|v| v.split())
}
