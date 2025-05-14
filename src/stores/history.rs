use leptos::prelude::{provide_context, use_context, ReadSignal, RwSignal, WriteSignal};
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

#[derive(Debug, Clone)]
pub struct History {
    /// A persistent, dedup-on-push command history (no consecutive duplicates)
    commands: Vec<String>,
    /// A temporary, clearable store for timestamped history entries.
    buffer: Vec<Entry>,
}

impl History {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn push(&mut self, input: impl Into<String>) {
        let input = input.into();

        self.buffer.push(Entry::new(input.clone()));

        // only push if commands is empty (i.e., .last() returns None)
        // or the last command is not the same as the new command
        if self.commands.last().is_none_or(|last| *last != input) {
            self.commands.push(input);
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn commands(&self) -> &[String] {
        &self.commands
    }

    pub fn buffer(&self) -> &[Entry] {
        &self.buffer
    }
}

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
