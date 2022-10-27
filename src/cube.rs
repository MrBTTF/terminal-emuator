use anyhow::Ok;
use nalgebra::Matrix4;
use render_gl_derive::VertexAttribPointers;

use crate::{
    render_gl::{
        self,
        buffer::{self, ArrayBuffer, ElementArrayBuffer, VertexArray},
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
    clr: data::u2_u10_u10_u10_rev_float,
    #[location = 2]
    tex: data::f32_f32,
}

pub struct Cube {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    ebo: buffer::ElementArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
    textures: Vec<Texture2D>,
    trig_count: i32,
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
}

impl Cube {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Cube, anyhow::Error> {
        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 0.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, 0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 0.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 0.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 0.0).into(),
            },
            Vertex {
                pos: (-0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (-0.5, 0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 0.0).into(),
            },
            Vertex {
                pos: (-0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 0.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 0.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, 0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            },
            Vertex {
                pos: (-0.5, 0.5, 0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 0.0).into(),
            },
            Vertex {
                pos: (-0.5, 0.5, -0.5).into(),
                clr: (0.0, 0.0, 0.0, 1.0).into(),
                tex: (0.0, 1.).into(),
            },
        ];
        let indices: Vec<i32> = (0..vertices.len()).map(|x| x as i32).collect();

        let vbo = ArrayBuffer::new(gl);
        let ebo = ElementArrayBuffer::new(gl);
        let vao = VertexArray::new(gl);
        let textures = vec![
            Texture2D::from_res(gl, res, "textures/awesomeface.png", gl::RGBA)?,
            Texture2D::from_res(gl, res, "textures/container.jpg", gl::RGB)?,
        ];

        program.set_used();
        for (i, tex) in textures.iter().enumerate() {
            tex.bind();
            tex.generate();
            let name = format!("texture{}", i);
            program.set_int(&name, i as i32)
        }

        vao.bind();
        vbo.bind();
        vbo.static_draw_data(&vertices);
        ebo.bind();
        ebo.static_draw_data(&indices);
        Vertex::vertex_attrib_pointers(&gl);
        ebo.unbind();
        vbo.unbind();
        vao.unbind();
        for tex in &textures {
            tex.unbind()
        }

        Ok(Cube {
            program,
            _vbo: vbo,
            ebo,
            vao,
            textures,
            trig_count: indices.len() as i32,
            model: Matrix4::identity(),
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
        })
    }

    pub fn set_model(&mut self, model: Matrix4<f32>) {
        self.model = model;
    }
    pub fn set_view(&mut self, view: Matrix4<f32>) {
        self.view = view;
    }
    pub fn set_projection(&mut self, projection: Matrix4<f32>) {
        self.projection = projection;
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();

        self.program.set_matrix("model", self.model);
        self.program.set_matrix("view", self.view);
        self.program.set_matrix("projection", self.projection);

        for (i, tex) in self.textures.iter().enumerate() {
            tex.activate(gl::TEXTURE0 + i as u32);
            tex.bind();
        }

        self.vao.bind();
        self.ebo.bind();

        unsafe {
            gl.DrawElements(
                gl::TRIANGLES, // mode
                self.trig_count,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }
    }
}
