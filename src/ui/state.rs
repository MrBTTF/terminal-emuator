use gl::Gl;

use crate::processor::Processor;

use super::textfield::TextField;

#[derive(Debug, Clone)]
pub enum Component {
    TextField,
    Processor,
}

#[derive(Debug, Clone)]
pub enum OutputEvent {
    Print(String),
    InputPos(u32),
    Newline,
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    UserText(String),
}

#[derive(Debug, Clone)]
pub enum Event {
    OutputEvent(OutputEvent),
    InputEvent(InputEvent),
}

pub struct State {
    text_field: TextField,
    processor: Processor,
}

impl State {
    pub fn new(text_field: TextField, processor: Processor) -> Self {
        State { text_field, processor }
    }

    pub fn init(&mut self) {
        self.processor.init();
    }

    pub fn update(&mut self, event: &glutin::event::Event<()>) {
        self.text_field.handle_window_event(event);

        for e in self.text_field.pop_events().into_iter() {
            self.processor.update(e);
        }

        for e in self.processor.pop_events().into_iter() {
            self.text_field.handle_output_event(e);
        }
    }

    pub fn render(&mut self, gl: &Gl) {
        self.text_field.render(gl);
    }
}
