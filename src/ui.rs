use gl::Gl;
use glutin::event::WindowEvent;

use crate::{graphics::cursor::Cursor, resources::Resources};

use self::textdisplay::{Buffer, TextDisplay};

pub mod textdisplay;
pub mod textfield;
pub mod state;

const GREEN: (u8, u8, u8, u8) = (0, 227, 48, 255);

// pub enum Event {
//     WindowEvent,
//     InputEvent,
// }

pub struct Ui {
    textdisplay: TextDisplay,
    cursor: Cursor,
}

impl Ui {
    pub fn new(
        res: &Resources,
        gl: &gl::Gl,
        width: u32,
        height: u32,
    ) -> Result<Self, anyhow::Error> {
        let color = GREEN;
        let textdisplay = TextDisplay::new(res, gl, width, height, color)?;
        let cursor = Cursor::new(
            res,
            gl,
            textdisplay.line_height as f32 * 0.6,
            textdisplay.line_height as f32,
            width as f32,
            height as f32,
            color,
        )?;

        anyhow::Ok(Ui { textdisplay, cursor })
    }

    pub fn handle_event(&mut self, event: &glutin::event::Event<()>) {
        if let glutin::event::Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::Resized(physical_size) => {
                    self.update_size(physical_size.width as i32, physical_size.height as i32);
                }
                _ => (),
            }
        }
    }

    pub fn update(&mut self, buffer: &mut Buffer) {
        self.textdisplay.update(buffer);
    }

    pub fn render(&mut self) {
        self.textdisplay.render();
    }

    fn update_size(&mut self, width: i32, height: i32) {
        self.textdisplay.update_size(width, height);
        self.textdisplay.render();
    }
}
