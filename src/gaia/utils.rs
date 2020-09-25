use gl;
use image;
use image::DynamicImage::*;
use image::*;
use image2::{
    ImageBuf,
    ImagePtr,
    Rgb,Rgba, Gray,
    Type,
    io,
    Filter,
    filter::ToGrayscale
 };
use image2::image::Image;
use std::os::raw::c_void;
use std::path::Path;


pub unsafe fn load_texture2(path: &str, file_format: &str)  -> u32 {
    println!("[stb]Loading texture from path: {} with format {}", path, file_format);
    let mut id = 0;


    gl::GenTextures(1, &mut id);
    let (data, dim, format) = match file_format {
        "png" => {
            let img : ImagePtr<u8, Rgba> = io::read_u8(path).unwrap();

            let img_data = img.data().to_vec();
            let (x,y,_) = img.shape();

            (img_data,(x as i32,y as i32),gl::RGBA)
        },
        _ => {
            let img : ImagePtr<u8, Rgb> = io::read_u8(path).unwrap();

            let img_data = img.data().to_vec();
            let (x,y,_) = img.shape();

            (img_data,(x as i32,y as i32),gl::RGB)
        },
        // _ => (io::read(path).unwrap() as ImageBuf<f64, Rgba>,gl::RGBA),
        // _ => (io::read_u8(path).unwrap() as ImagePtr<u8, Rgb>,gl::RGB),
    };

    gl::BindTexture(gl::TEXTURE_2D, id);
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

    id
}


pub unsafe fn load_texture(path: &str) -> u32 {
    println!("Loading texture from path: {}", path);
    let mut id = 0;

    gl::GenTextures(1, &mut id);
    let img = image::open(&Path::new(path)).expect("Texture failed to load");
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => {println!("RGB for {}",path);
            gl::RGB},
        ImageRgba8(_) => {println!("RGBA for {}",path);
            gl::RGBA},
        _ => {println!("RGBA for {}",path);
        gl::RGB},
    };
    let data = img.raw_pixels();
    let dim = img.dimensions();

    gl::BindTexture(gl::TEXTURE_2D, id);
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

    id
}

pub unsafe fn load_texture_from_dir(filename: &str, directory: &str) -> u32 {
    let fullpath = format!("{}/{}", directory, filename);
    let dot = filename.find(".").unwrap_or_default() + 1usize;
    let (_, format) = filename.split_at(dot);

    load_texture2(&fullpath,format)
}
