use anyhow::Ok;
use render_gl_derive::VertexAttribPointers;
use rusttype::{point, Font, Point, Scale};

use crate::{
    render_gl::{
        self,
        buffer::{self, ArrayBuffer, VertexArray},
        data,
        texture::Texture2D,
    },
    resources::Resources,
};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    tex: data::f32_f32,
}

pub struct RenderText {
    gl: gl::Gl,
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    texture: Texture2D,
    font: Font<'static>,
    scale: Scale,
    pub glyph_width: u32,
    pub glyph_height: u32,
    offset: Point<f32>,
    color: (u8, u8, u8, u8),
}

impl RenderText {
    pub fn new(
        res: &Resources,
        gl: &gl::Gl,
        width: u32,
        height: u32,
        color: (u8, u8, u8, u8),
    ) -> Result<RenderText, anyhow::Error> {
        let program = render_gl::Program::from_res(gl, res, "shaders/rendertext")?;

        #[rustfmt::skip]
        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (-1.0, -1.0, 0.0).into(), tex: (0.0, 0.0).into() }, // bottom left
            Vertex { pos: (1.0, -1.0, 0.0).into(), tex: (1.0, 0.0).into() },  // bottom right
            Vertex { pos: (1.0, 1.0, 0.0).into(), tex: (1.0, 1.0).into() },   // top right
            Vertex { pos: (1.0, 1.0, 0.0).into(), tex: (1.0, 1.0).into() },   // top right
            Vertex { pos: (-1.0, 1.0, 0.0).into(), tex: (0.0, 1.0).into() },  // top left
            Vertex { pos: (-1.0, -1.0, 0.0).into(), tex: (0.0, 0.0).into() }, // bottom left
        ];

        let texture = Texture2D::new_rgba(gl, width, height);

        texture.bind();
        texture.generate();

        let vbo = ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = VertexArray::new(gl);
        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();
        texture.unbind();

        let font = res.load_font("fonts/Modeseven.ttf")?;
        let scale = Scale::uniform(30.0);
        let v_metrics = font.v_metrics(scale);
        let glyph_width = font
            .layout("a", scale, point(0.0, 0.0))
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .last()
            .unwrap_or(0.0) as u32;
        let glyph_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let offset = point(0.0, v_metrics.ascent);

        Ok(RenderText {
            gl: gl.clone(),
            program,
            _vbo: vbo,
            vao,
            texture,
            font,
            scale,
            glyph_width,
            glyph_height,
            offset,
            color,
        })
    }

    fn render_text(&mut self, text: &str, line_no: usize) {
        let glyphs: Vec<_> = self.font.layout(text, self.scale, self.offset).collect();

        let width = self.texture.image_res.width;
        let height = self.texture.image_res.height;

        for g in glyphs {
            // if g.id() == rusttype::GlyphId(0) {
            //     println!("{:?}", g);
            //     continue;
            // }
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    // println!("{:?}", g);
                    let x = x + bb.min.x as u32;
                    if x >= width {
                        return;
                    }

                    let y = height.saturating_sub(y + bb.min.y as u32);
                    let y = y.saturating_sub(line_no as u32 * self.glyph_height);

                    let idx = (4 * (x + y * width)) as usize;
                    // if idx >= self.texture.image_res.data.len() {
                    //     return;
                    // }
                    self.texture.image_res.data[idx..idx + 4].clone_from_slice(&[
                        self.color.0,
                        self.color.1,
                        self.color.2,
                        (v * self.color.3 as f32) as u8,
                    ]);
                })
            }
        }
    }

    fn render_lines(&mut self, lines: &[String]) {
        self.texture.image_res.data = vec![0; self.texture.image_res.data.len()];

        for (line_no, line) in lines.iter().enumerate() {
            self.render_text(line, line_no)
        }
    }

    pub fn update_size(&mut self, width: i32, height: i32) {
        self.texture = Texture2D::new_rgba(&self.gl, width as u32, height as u32);
    }

    pub fn update(&mut self, lines: &[String]) {
        self.render_lines(lines);
        self.texture.bind();
        self.texture.generate();
        self.texture.unbind();
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();

        self.texture.bind();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6,             // number of indices to be rendered
            );
        }
    }
}
