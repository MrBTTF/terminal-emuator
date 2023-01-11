use crate::{
    processor::process,
    ui::{textdisplay::Buffer, Ui},
};

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Resized(u32, u32),
    ReceivedCharacter(char),
    Backspace,
    Enter,
    Left,
    Right,
    Previous,
    Next,
    Release,
}

pub struct Shell {
    ui: Ui,
    history: Vec<String>,
    input: String,
    cmd_history: Vec<String>,
    cmd_pointer: usize,
}

impl Shell {
    pub fn new(ui: Ui) -> Result<Self, anyhow::Error> {
        let username = "user".to_string();
        let directory = "/".to_string();
        let mut shell = Shell {
            ui,
            history: vec![format!("{}:{}$ ", username, directory)],
            input: String::new(),
            cmd_history: vec![],
            cmd_pointer: 0,
        };
        shell.draw_buffer();
        shell.move_cursor_to_end();
        anyhow::Ok(shell)
    }

    pub fn handle_event(&mut self, event: Event) {
        self.ui.handle_event(event);
        match event {
            Event::Resized(_, _) => {
                self.draw_buffer();
            }
            Event::ReceivedCharacter(c) => {
                self.input.push(c);
                self.shift_cursor(1);
                self.draw_buffer();
            }
            Event::Backspace => {
                self.shift_cursor(-1);
                self.input.pop();
                self.draw_buffer();
            }
            Event::Enter => {
                if let Some(last) = self.history.last_mut() {
                    last.push_str(&self.input);
                } else {
                    self.history.push(self.input.clone());
                }
                self.cmd_history.push(self.input.clone());
                self.cmd_pointer += 1;
                let output = self.process_cmd(&self.input);
                self.history.extend(output);

                self.input.clear();
                self.draw_buffer();
                self.move_cursor_to_end();
            }
            Event::Left => self.shift_cursor(-1),
            Event::Right => self.shift_cursor(1),
            Event::Previous => self.previous_input(),
            Event::Next => self.next_input(),
            _ => (),
        }
        self.ui.update_cursor(&self.history);
    }

    pub fn update(&mut self) {
        self.ui.render();
    }

    fn draw_buffer(&mut self) {
        let buffer = Buffer::new(self.history.clone(), &self.input);
        self.ui.update_text(&buffer);
    }

    fn shift_cursor(&mut self, shift: i32) {
        self.ui.shift_cursor(shift, self.input.len());
    }

    fn move_cursor_to_end(&mut self) {
        self.shift_cursor(self.input.len() as i32);
    }

    fn previous_input(&mut self) {
        if self.cmd_pointer == 0 {
            return;
        }
        self.cmd_pointer -= 1;
        self.input = self.cmd_history[self.cmd_pointer].clone();
        self.draw_buffer();
        self.move_cursor_to_end();
    }

    fn next_input(&mut self) {
        if self.cmd_pointer == self.cmd_history.len() - 1 {
            return;
        }
        self.cmd_pointer += 1;
        self.input = self.cmd_history[self.cmd_pointer].clone();
        self.draw_buffer();
        self.move_cursor_to_end();
    }

    fn process_cmd(&self, input: &str) -> Vec<String> {
        let mut output = vec![];

        let mut input_splitted = input.split_whitespace();
        if let Some(cmd) = input_splitted.next() {
            let args = input_splitted;
            dbg!(cmd);
            match cmd {
                "echo" => {
                    output = vec![args.fold(String::new(), |r, c| r + c + " ")];
                    output.push(String::new());
                }
                "ls" => {
                    output = vec!["bin    dev    usr\n".to_string()];
                    output.push(String::new());
                }
                _ => {
                    output = process(input);
                    // output = vec![format!("Command '{cmd}' not found"), String::new()];
                }
            }
        } else {
            output.push(String::new());
        }

        let username = "user".to_string();
        let directory = "/".to_string();
        let cwd = format!("{}:{}$ ", username, directory);
        if let Some(last) = output.last_mut() {
            last.push_str(&cwd);
        }
        output
    }
}
