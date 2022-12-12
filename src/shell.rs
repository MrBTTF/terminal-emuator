use gl::Gl;
use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::{
    resources::Resources,
    ui::{
        textdisplay::{Buffer, TextDisplay},
        Ui,
    },
};

const GREEN: (u8, u8, u8, u8) = (0, 227, 48, 255);

pub struct Shell {
    ui: Ui,
    history: Vec<String>,
    input: String,
}

impl Shell {
    pub fn new(ui: Ui) -> Result<Self, anyhow::Error> {
        anyhow::Ok(Shell { ui, history: Vec::new(), input: String::new() })
    }

    pub fn handle_event(&mut self, event: &glutin::event::Event<()>) {
        self.ui.handle_event(event);
        if let glutin::event::Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::Resized(physical_size) => {
                    self.draw_buffer();
                }
                WindowEvent::ReceivedCharacter(c) => {
                    // println!("{}, {:#?}", c, c);
                    match c {
                        '\u{8}' | '\r' => (), //backspace
                        _ => {
                            self.input.push(*c);
                            self.draw_buffer();
                            // println!("{:#?}", &self.input);
                        }
                    }
                }
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    is_synthetic: _,
                } => {
                    match keycode {
                        VirtualKeyCode::Back => {
                            self.input.pop();
                        }
                        VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {
                            if let Some(last) = self.history.last_mut() {
                                last.push_str(&self.input);
                            } else {
                                self.history.push(self.input.clone());
                            }
                            self.input.clear();

                            let username = "user".to_string();
                            let directory = "/".to_string();
                            self.history.push(format!("{}:{}$ ", username, directory));
                            self.draw_buffer();
                        }
                        _ => (),
                    };
                }
                _ => (),
            }
        }
    }

    pub fn update(&mut self) {
        self.ui.render();
    }

    fn draw_buffer(&mut self) {
        let mut buffer = Buffer::new(self.history.clone(), &self.input);
        // println!("{:#?}", self.history);
        // println!("{:#?}", self.input);
        self.ui.update(&mut buffer);
    }
}
