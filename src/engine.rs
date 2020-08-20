use std::os::raw::c_void;
use std::path::Path;
use std::sync::mpsc::Receiver;

use gl;
extern crate glfw;
use self::glfw::{Action, Key};

use image;
use image::DynamicImage::*;
use image::GenericImage;
use image::*;

pub unsafe fn loadTexture(path: &str) -> u32 {
    let mut textureID = 0;

    gl::GenTextures(1, &mut textureID);
    let img = image::open(&Path::new(path)).expect("Texture failed to load");
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
        _ => gl::RGB,
    };

    let data = img.raw_pixels();
    let dim = img.dimensions();

    gl::BindTexture(gl::TEXTURE_2D, textureID);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        format as i32,
        dim.0 as i32,
        dim.1 as i32,
        0,
        format,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR as i32,
    );
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    textureID
}

pub unsafe fn loadTextureFromDir(filename: &str, directory: &str) -> u32 {
    let fullpath = format!("{}/{}", directory, filename);

    loadTexture(&fullpath)
}