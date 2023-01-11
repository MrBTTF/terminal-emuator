use std::time::Instant;

use anyhow::Ok;
use nalgebra::{Matrix4, Vector3};
use render_gl_derive::VertexAttribPointers;

use crate::{
    render_gl::{
        self,
        buffer::{self, ArrayBuffer, VertexArray},
        data,
    },
    resources::Resources,
};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Cursor {
    gl: gl::Gl,
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,

    x: u32,
    y: u32,
    width: f32,
    height: f32,
    field_width: f32,
    field_height: f32,

    to_screen: Matrix4<f32>,
    model: Matrix4<f32>,
}

impl Cursor {
    pub fn new(
        res: &Resources,
        gl: &gl::Gl,
        width: f32,
        height: f32,
        screen_width: f32,
        screen_height: f32,
        color: (u8, u8, u8, u8),
    ) -> Result<Cursor, anyhow::Error> {
        let program = render_gl::Program::from_res(gl, res, "shaders/cursor")?;

        let color = (
            color.0 as f32 / 255.,
            color.1 as f32 / 255.,
            color.2 as f32 / 255.,
            color.3 as f32 / 255.,
        );

        #[rustfmt::skip]
        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (0.0, 0.0, 0.0).into(), clr: color.into() },
            Vertex { pos: (1.0, 0.0, 0.0).into(), clr: color.into() },
            Vertex { pos: (1.0, 1.0, 0.0).into(), clr: color.into() },

            Vertex { pos: (1.0, 1.0, 0.0).into(), clr: color.into() },
            Vertex { pos: (0.0 ,1.0, 0.0).into(), clr: color.into() },
            Vertex { pos: (0.0, 0.0, 0.0).into(), clr: color.into() },
        ];

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

        let field_width = screen_width / width;
        let field_height = screen_height / height;
        let to_screen: Matrix4<f32> = Matrix4::identity()
            .append_nonuniform_scaling(&Vector3::new(
                1. / (field_width / 2.),
                1. / (field_height / 2.),
                1.,
            ))
            .append_translation(&Vector3::new(-1.0, -1.0, 0.0))
            .append_nonuniform_scaling(&Vector3::new(1.0, -1.0, 0.0));

        Ok(Cursor {
            gl: gl.clone(),
            program,
            _vbo: vbo,
            vao,
            width,
            height,
            field_width,
            field_height,
            to_screen,
            model: Matrix4::identity(),
            x: 0,
            y: 0,
        })
    }

    pub fn move_to(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
        self.model = Matrix4::new_translation(&Vector3::new(x as f32, y as f32, 0.0));
    }

    pub fn update_size(&mut self, screen_width: f32, screen_height: f32) {
        let new_field_width = screen_width / self.width;
        let new_field_height = screen_height / self.height;
        self.to_screen.prepend_translation_mut(&Vector3::new(
            self.field_width,
            -self.field_height,
            0.0,
        ));
        self.to_screen.prepend_nonuniform_scaling_mut(&Vector3::new(
            self.field_width as f32 / new_field_width,
            self.field_height as f32 / new_field_height,
            1.0,
        ));

        self.field_width = new_field_width;
        self.field_height = new_field_height;
        self.to_screen.prepend_translation_mut(&Vector3::new(
            -self.field_width,
            self.field_height,
            0.0,
        ));
    }

    pub fn render(&mut self) {
        self.program.set_used();

        self.program.set_matrix("to_screen", self.to_screen);
        self.program.set_matrix("model", self.model);
        self.vao.bind();
        unsafe {
            self.gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6,             // number of indices to be rendered
            );
        }
    }
}
