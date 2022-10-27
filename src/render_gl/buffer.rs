

use gl;

pub type ArrayBuffer = Buffer<{ gl::ARRAY_BUFFER }>;
pub type ElementArrayBuffer = Buffer<{ gl::ELEMENT_ARRAY_BUFFER }>;
pub struct Buffer<const BUFFER_TYPE: gl::types::GLuint> {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
}

impl<const BUFFER_TYPE: gl::types::GLuint> Buffer<BUFFER_TYPE> {
    pub fn new(gl: &gl::Gl) -> Buffer<BUFFER_TYPE> {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }

        Buffer { gl: gl.clone(), vbo }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(BUFFER_TYPE, 0);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                BUFFER_TYPE,                                                        // target
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                           // usage
            );
        }
    }
}

impl<const BUFFER_TYPE: gl::types::GLuint> Drop for Buffer<BUFFER_TYPE> {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.vbo);
        }
    }
}
pub struct VertexArray {
    gl: gl::Gl,
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        VertexArray { gl: gl.clone(), vao }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}
