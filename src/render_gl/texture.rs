

use gl;
use thiserror::Error;

use crate::resources::{self, ImageResource, Resources};

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
}

pub struct Texture2D {
    gl: gl::Gl,
    texture: gl::types::GLuint,
    pub image_res: ImageResource,
    format: gl::types::GLenum,
}

impl Texture2D {
    pub fn new(gl: &gl::Gl, image_res: ImageResource, format: gl::types::GLenum) -> Texture2D {
        let mut texture: gl::types::GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
        }

        Texture2D { gl: gl.clone(), texture, image_res, format }
    }

    pub fn from_res(
        gl: &gl::Gl,
        res: &Resources,
        name: &str,
        format: gl::types::GLenum,
    ) -> Result<Texture2D, Error> {
        let img_res = res
            .load_image(name)
            .map_err(|e| Error::ResourceLoad { name: name.into(), inner: e })?;

        Ok(Texture2D::new(gl, img_res, format))
    }

    pub fn new_rgba(gl: &gl::Gl, width: u32, height: u32) -> Texture2D {
        let data = vec![0; (width * height * 4) as usize];
        let img_res = ImageResource { data, width, height };
        Texture2D::new(gl, img_res, gl::RGBA)
    }

    pub fn generate(&self) {
        unsafe {
            self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            self.gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                self.format as i32,
                self.image_res.width as i32,
                self.image_res.height as i32,
                0,
                self.format,
                gl::UNSIGNED_BYTE,
                self.image_res.data.as_ptr() as *const gl::types::GLvoid,
            );
            self.gl.GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn update(&self, data: Vec<u8>) {
        unsafe {
            self.gl.TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                self.image_res.width as i32,
                self.image_res.height as i32,
                self.format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const gl::types::GLvoid,
            );
        }
    }

    pub fn activate(&self, unit: gl::types::GLenum) {
        unsafe {
            self.gl.ActiveTexture(unit);
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

// impl Drop for Texture2D {
//     fn drop(&mut self) {
//         unsafe {
//             self.gl.DeleteVertexArrays(1, &self.texture);
//         }
//     }
// }
