use leptos::prelude::{ReadSignal, RwSignal, WriteSignal, provide_context, use_context};

#[derive(Debug, Clone)]
pub struct Entry {
    timestamp: u64,
    pub input: String,
}

impl Entry {
    pub fn new(input: String) -> Self {
        // use SystemTime for testing as js_sys::Date is only available in wasm target
        #[cfg(test)]
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        #[cfg(not(test))]
        let timestamp = web_sys::js_sys::Date::now() as u64;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_new() {
        let history = History::new();
        assert!(history.commands().is_empty());
        assert!(history.buffer().is_empty());
    }

    #[test]
    fn test_history_push_basic() {
        let mut history = History::new();
        history.push("echo");

        assert_eq!(history.commands(), &["echo"]);
        assert_eq!(
            history
                .buffer()
                .iter()
                .map(|e| e.input.clone())
                .collect::<Vec<_>>(),
            &["echo"]
        );
    }

    #[test]
    fn test_history_push_duplicate() {
        let mut history = History::new();
        history.push("echo");
        history.push("echo");

        assert_eq!(history.commands(), &["echo"]);
        assert_eq!(
            history
                .buffer()
                .iter()
                .map(|e| e.input.clone())
                .collect::<Vec<_>>(),
            &["echo", "echo"]
        );
    }

    #[test]
    fn test_history_push_different() {
        let mut history = History::new();
        history.push("echo");
        history.push("clear");

        assert_eq!(history.commands(), &["echo", "clear"]);
        assert_eq!(
            history
                .buffer()
                .iter()
                .map(|e| e.input.clone())
                .collect::<Vec<_>>(),
            &["echo", "clear"]
        );
    }

    #[test]
    fn test_history_clear() {
        let mut history = History::new();
        history.push("echo");
        history.push("clear");

        history.clear();
        assert!(history.buffer().is_empty());
        assert_eq!(history.commands(), &["echo", "clear"]); // commands should remain
    }
}
