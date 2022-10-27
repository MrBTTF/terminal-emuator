use image::io::Reader as ImageReader;
use rusttype::Font;

use std::ffi;
use std::fs;

use std::io::{self, Read};
use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(
        #[source]
        #[from]
        io::Error,
    ),
    #[error("ImageError error")]
    ImageError(
        #[source]
        #[from]
        image::ImageError,
    ),
    #[error("Failed to read CString from file that contains 0")]
    FileContainsNil,
    #[error("Failed get executable path")]
    FailedToGetExePath,
    #[error("Invald path")]
    InvalidPath,
}

pub struct ImageResource {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        let exe_file_name = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;

        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;
        Ok(Resources {
            root_path: exe_path.join(rel_path),
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;

        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        // check for null byte
        if buffer.iter().any(|i| *i == 0) {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    pub fn load_image(&self, resource_name: &str) -> Result<ImageResource, Error> {
        let mut img =
            ImageReader::open(resource_name_to_path(&self.root_path, resource_name))?.decode()?;
        img = img.flipv();
        Ok(ImageResource {
            data: img.clone().into_bytes(),
            width: img.width(),
            height: img.height(),
        })
    }

    pub fn load_font(&self, resource_name: &str) -> Result<Font<'static>, Error> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;

        let mut font_data: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut font_data)?;
        let font = Font::try_from_vec(font_data).expect("error constructing a Font from bytes");
        Ok(font)
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split('/') {
        path = path.join(part);
    }

    path
}
