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

        if let Some(e) = self.processor.output_event.first() {
            self.text_field.handle_output_event(e.clone());
            self.processor.output_event.remove(0);
        }

        if let Some(e) = self.text_field.input_event.first() {
            self.processor.update(e.clone());
            self.text_field.input_event.remove(0);
        }
    }

    pub fn render(&mut self, gl: &Gl) {
        self.text_field.render(gl);
    }
}
