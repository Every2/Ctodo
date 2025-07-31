use iced::{
    Alignment::Center,
    Length::Fill,
    widget::{button, checkbox, row, text, text_input},
    window,
};

use iced::Element;

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

#[derive(Clone)]
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
    fn matches(self, task: &Task) -> bool {
        match self {
            Filters::All => true,
            Filters::Active => !task.completed,
            Filters::Completed => !task.completed,
        }
    }
}

impl Default for Filters {
    fn default() -> Self {
        Self::All
    }
}

impl Task {
    fn new(desciption: String) -> Self {
        Self {
            index: 0,
            description: desciption,
            completed: false,
            state: TaskState::Idle,
        }
    }

    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("task-{i}"))
    }

    fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Completed(completed) => {
                self.completed = completed;
            }
            TaskMessage::Edit => {
                self.state = TaskState::Editing;
            }
            TaskMessage::DescriptionEdited(new_description) => {
                self.description = new_description;
            }
            TaskMessage::FinishEdition => {
                if !self.description.is_empty() {
                    self.state = TaskState::Idle;
                }
            }
            TaskMessage::Delete => {}
        }
    }

    fn view(&self, i: usize) -> Element<'_, TaskMessage> {
        match &self.state {
            TaskState::Idle => {
                let checkbox = checkbox(&self.description, self.completed)
                    .on_toggle(TaskMessage::Completed)
                    .width(Fill)
                    .size(17)
                    .text_shaping(text::Shaping::Advanced);

                row![
                    checkbox,
                    button("change")
                        .on_press(TaskMessage::Edit)
                        .padding(10)
                        .style(button::text)
                ]
                .spacing(20)
                .align_y(Center)
                .into()
            }
            TaskState::Editing => {
                let text_input = text_input("Describe your task...", &self.description)
                    .id(Self::text_input_id(i))
                    .on_input(TaskMessage::DescriptionEdited)
                    .on_submit(TaskMessage::FinishEdition)
                    .padding(10);

                row![
                    text_input,
                    button(row!["delete", "Delete"].spacing(10).align_y(Center))
                        .on_press(TaskMessage::Delete)
                        .padding(10)
                        .style(button::danger)
                ]
                .spacing(20)
                .align_y(Center)
                .into()
            }
        }
    }
}
