use crate::{graphics::cursor::Cursor, resources::Resources, shell::Event};

use self::textdisplay::{Buffer, TextDisplay};

pub mod textdisplay;

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

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Resized(width, height) => {
                self.update_size(width as i32, height as i32);
            }
            _ => (),
        }
    }

    pub fn update(&mut self, buffer: &Buffer) {
        self.textdisplay.update(buffer);
        self.move_cursor_to_end(buffer);
    }

    pub fn render(&mut self) {
        self.textdisplay.render();
        self.cursor.render();
    }

    fn update_size(&mut self, width: i32, height: i32) {
        self.textdisplay.update_size(width, height);
        self.cursor.update_size(width as f32, height as f32);
    }

    fn move_cursor_to_end(&mut self, buffer: &Buffer) {
        if self.textdisplay.get_line_width() == 0 {
            return;
        }
        if let Some(current_line) = buffer.content().last() {
            // println!("line count: {}", self.textdisplay.get_lines_count());
            let mut y = self.textdisplay.get_lines_count().saturating_sub(1);
            if let Some(last_line) = buffer.content().last() {
                // println!("{}, {}", last_line.len(), self.textdisplay.get_line_width());
                if !last_line.is_empty() && last_line.len() % self.textdisplay.get_line_width() == 0
                {
                    y += 1;
                }
            }
            // println!("x: {}", current_line.len() % self.textdisplay.get_line_width());
            // println!("y: {}", y);
            self.cursor
                .move_to((current_line.len() % self.textdisplay.get_line_width()) as u32, y as u32);
        }
    }
}
