use gl;
use nalgebra::Matrix4;
use thiserror::Error;

use std::ffi::{CStr, CString};

use gl::types::{GLchar, GLenum, GLint, GLuint};

use crate::resources::{self, Resources};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to load resource {name}")]
    ResourceLoad {
        name: String,
        #[source]
        inner: resources::Error,
    },
    #[error("Can not determine shader type for resource {name}")]
    CanNotDetermineShaderTypeForResource { name: String },
    #[error("Failed to compile shader {name}: {message}")]
    CompileError { name: String, message: String },
    #[error("Failed to link program {name}: {message}")]
    LinkError { name: String, message: String },
}

pub struct Program {
    gl: gl::Gl,
    id: GLuint,
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl.LinkProgram(program_id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl.DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { gl: gl.clone(), id: program_id })
    }

    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

        let resource_names = POSSIBLE_EXT
            .iter()
            .map(|file_extension| format!("{}{}", name, file_extension))
            .collect::<Vec<String>>();

        let shaders = resource_names
            .iter()
            .map(|resource_name| Shader::from_res(gl, res, resource_name))
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(gl, &shaders[..])
            .map_err(|message| Error::LinkError { name: name.into(), message })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let cname = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe {
            self.gl.Uniform1i(
                self.gl.GetUniformLocation(self.id, cname.as_ptr() as *const gl::types::GLchar),
                value,
            );
        }
    }

    pub fn set_matrix(&self, name: &str, mat: Matrix4<f32>) {
        let cname = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe {
            self.gl.UniformMatrix4fv(
                self.gl.GetUniformLocation(self.id, cname.as_ptr()),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }

    pub fn get_active_uniforms(&self) -> i32 {
        let mut params = 0;
        unsafe {
            self.gl.GetProgramiv(
                self.id,
                gl::ACTIVE_UNIFORMS,
                &mut params as *mut gl::types::GLint,
            );
        }

        params
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    gl: gl::Gl,
    id: GLuint,
}

impl Shader {
    pub fn from_source(gl: &gl::Gl, source: &CStr, shader_type: GLenum) -> Result<Shader, String> {
        let id = shader_from_source(gl, source, shader_type)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, GLenum); 2] =
            [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

        let shader_type = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name.into() })?;

        let source = res
            .load_cstring(name)
            .map_err(|e| Error::ResourceLoad { name: name.into(), inner: e })?;
        Shader::from_source(gl, &source, shader_type)
            .map_err(|message| Error::LinkError { name: name.into(), message })
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn shader_from_source(gl: &gl::Gl, source: &CStr, shader_type: GLuint) -> Result<GLuint, String> {
    let id = unsafe { gl.CreateShader(shader_type) };

    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error: CString = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl.GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
        }
        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let buffer: Vec<u8> = vec![b' '; len as usize + 1];
    unsafe { CString::from_vec_unchecked(buffer) }
}
