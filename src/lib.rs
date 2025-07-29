use iced::window;

enum Ctodo {
    Standard,
    Changed(State),
}

enum Filters {
    All,
    Active,
    Completed,
}

struct State {
    input_value: String,
    filter: Filters,
    tasks: Vec<Task>,
    not_saved: bool,
    saving: bool,
}

enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTask,
    FilterChanged(Filters),
    TaskMessage(usize, TaskMessage),
    ToggleFullSCREEN(window::Mode),
}

struct Task {
    index: u32,
    description: String,
    completed: bool,
    state: TaskState,
}

enum TaskState {
    Idle,
    Editing,
}

enum TaskMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishEdition,
    Delete,
}

struct SavedState {
    input_value: String,
    filter: Filters,
    tasks: Vec<Task>,
}

enum LoadError {
    File,
    Format,
}

enum SaveError {
    Write,
    Format,
}

impl Filters {
    fn matches(self) -> bool {
        match self {
            Filters::All => true,
            Filters::Active => false,
            Filters::Completed => false,
        }
    }
}

impl Default for Filters {
    fn default() -> Self {
        Self::All
    }
}
