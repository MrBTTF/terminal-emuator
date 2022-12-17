use gl::Gl;
use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::{
    resources::Resources,
    ui::{
        textdisplay::{Buffer, TextDisplay},
        Ui,
    },
};

#[derive(Copy, Clone)]
pub enum Event {
    Resized(u32, u32),
    ReceivedCharacter(char),
    Backspace,
    Enter,
}

pub struct Shell {
    ui: Ui,
    history: Vec<String>,
    input: String,
}

impl Shell {
    pub fn new(ui: Ui) -> Result<Self, anyhow::Error> {
        anyhow::Ok(Shell { ui, history: Vec::new(), input: String::new() })
    }

    pub fn handle_event(&mut self, event: Event) {
        self.ui.handle_event(event);
        match event {
            Event::Resized(_, _) => {
                self.draw_buffer();
            }
            Event::ReceivedCharacter(c) => {
                // println!("{}, {:#?}", c, c);
                self.input.push(c);
                self.draw_buffer();
                // println!("{:#?}", &self.input);
            }
            Event::Backspace => {
                self.input.pop();
                self.draw_buffer();
            }
            Event::Enter => {
                if let Some(last) = self.history.last_mut() {
                    last.push_str(&self.input);
                } else {
                    self.history.push(self.input.clone());
                }
                let output = self.process_cmd(&self.input);
                self.history.extend(output);

                self.input.clear();
                self.draw_buffer();
            }
            _ => (),
        }
    }

    pub fn update(&mut self) {
        self.ui.render();
    }

    fn draw_buffer(&mut self) {
        let buffer = Buffer::new(self.history.clone(), &self.input);
        // println!("{:#?}", self.history);
        // println!("{:#?}", self.input);
        self.ui.update(&buffer);
    }

    fn process_cmd(&self, input: &str) -> Vec<String> {
        let mut output = vec![];

        let mut input_splitted = input.split_whitespace();
        if let Some(cmd) = input_splitted.next() {
            let args = input_splitted;
            println!("cmd: {}", cmd);
            // println!("args: {:?}", args);
            match cmd {
                "echo" => output = vec![args.fold(String::new(), |r, c| r + c + " ")],
                "ls" => {
                    output = vec!["bin    dev    usr".to_string()];
                }
                _ => {
                    output = vec![format!("Command '{cmd}' not found")];
                }
            }
        }

        let username = "user".to_string();
        let directory = "/".to_string();
        let cwd = format!("{}:{}$ ", username, directory);
        output.push(cwd);
        output
    }
}
