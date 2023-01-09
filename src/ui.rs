use nalgebra::ComplexField;

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
    pub cursor_position: usize,
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

        anyhow::Ok(Ui { textdisplay, cursor, cursor_position: 0 })
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Resized(width, height) => {
                self.update_size(width as i32, height as i32);
            }
            _ => (),
        }
    }

    pub fn update_text(&mut self, buffer: &Buffer) {
        self.textdisplay.update(buffer);
    }

    pub fn render(&mut self) {
        self.textdisplay.render();
        self.cursor.render();
    }

    pub fn shift_cursor(&mut self, shift: i32, input_size: usize) {
        if input_size == 0 {
            self.cursor_position = 0;
            return;
        }
        let cursor_position = self.cursor_position as i32 + shift;
        if cursor_position > input_size as i32 {
            self.cursor_position = input_size;
        } else if cursor_position >= 0 {
            self.cursor_position = cursor_position as usize;
        }
        dbg!(input_size);
        dbg!(cursor_position);
    }

    pub fn update_cursor(&mut self, history: &[String]) {
        let line_width = self.textdisplay.get_line_width();

        let history_last_line_width = if let Some(history_last_line) = history.last() {
            history_last_line.len() % line_width
        } else {
            0
        };

        let mut history_last_line_y = history
            .iter()
            .fold(history.len() - 1, |acc, s| {
                // dbg!(acc);
                // dbg!(s.len());
                // dbg!((line_width as usize));
                acc + s.len() / (line_width as usize)
            }); // Adding remainder of each row if it's longer than line width

        dbg!(self.textdisplay.lines_to_display);
        if history_last_line_y > self.textdisplay.lines_to_display - 1  {
            history_last_line_y = self.textdisplay.lines_to_display - 1
        }
        let last_line_width = history_last_line_width + self.cursor_position;
        let new_x = last_line_width % line_width;
        let new_y = history_last_line_y + (last_line_width) / line_width;

        dbg!(self.cursor_position);
        dbg!(line_width);
        dbg!(history_last_line_width);
        dbg!(history_last_line_y);
        dbg!(last_line_width);
        dbg!(new_x);
        dbg!(new_y);

        self.cursor.move_to(new_x as u32, new_y as u32);
    }

    fn update_size(&mut self, width: i32, height: i32) {
        self.textdisplay.update_size(width, height);
        self.cursor.update_size(width as f32, height as f32);
    }

    pub fn move_cursor_to_end(&mut self, buffer: &Buffer) {
        if self.textdisplay.get_line_width() == 0 {
            return;
        }
        if let Some(current_line) = buffer.content().last() {
            // println!("line count: {}", self.textdisplay.get_lines_count());
            let mut y = self.textdisplay.get_lines_count().saturating_sub(1);
            if !current_line.is_empty()
                && current_line.len() % self.textdisplay.get_line_width() == 0
            {
                y += 1;
            }
            // println!("x: {}", current_line.len() % self.textdisplay.get_line_width());
            // println!("y: {}", y);
            self.cursor
                .move_to((current_line.len() % self.textdisplay.get_line_width()) as u32, y as u32);
        }
    }
}
