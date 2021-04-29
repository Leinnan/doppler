use gl;
use image2::image::Image;
use image2::{io, ImagePtr, Rgb, Rgba};
use log::{info,error};
use std::os::raw::c_void;

pub fn path_exists(path: &std::path::Path) -> bool {
    let meta = std::fs::metadata(path);
    meta.is_ok()
}

pub unsafe fn load_texture(path: &str, file_format: &str) -> u32 {
    info!("Loading texture: {}", path);
    let mut id = 0;

    gl::GenTextures(1, &mut id);
    let (data, dim, format) = match file_format {
        "png" => {
            let img: ImagePtr<u8, Rgba> = io::read_u8(path).unwrap();
            let img_data = img.data().to_vec();
            let (x, y, _) = img.shape();

            (img_data, (x as i32, y as i32), gl::RGBA)
        }
        _ => {
            let img: ImagePtr<u8, Rgb> = io::read_u8(path).unwrap();
            let img_data = img.data().to_vec();
            let (x, y, _) = img.shape();

            (img_data, (x as i32, y as i32), gl::RGB)
        }
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

pub unsafe fn load_texture_from_dir(filename: &str, directory: &str) -> u32 {
    let fullpath = format!("{}/{}", directory, filename);
    let dot = filename.find(".").unwrap_or_default() + 1usize;
    let (_, format) = filename.split_at(dot);

    load_texture(&fullpath, format)
}

/// loads a cubemap texture from 6 individual texture faces
/// order:
/// +X (right)
/// -X (left)
/// +Y (top)
/// -Y (bottom)
/// +Z (front)
/// -Z (back)
/// -------------------------------------------------------
pub unsafe fn load_cubemap(faces: &[&str]) -> u32 {
    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

    for (i, face) in faces.iter().enumerate() {
        if !path_exists(std::path::Path::new(&face)) {
            error!("There is no file {}", face);
        } else {
            info!("Loading {}", face);
        }
        let (data, dim) = {
            let img: ImagePtr<u8, Rgb> = io::read_u8(face).unwrap();
            let img_data = img.data().to_vec();
            let (x, y, _) = img.shape();

            (img_data, (x as i32, y as i32))
        };
        gl::TexImage2D(
            gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
            0,
            gl::RGB as i32,
            dim.0,
            dim.1,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );
    }

    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MAG_FILTER,
        gl::LINEAR as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_S,
        gl::CLAMP_TO_EDGE as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_T,
        gl::CLAMP_TO_EDGE as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_R,
        gl::CLAMP_TO_EDGE as i32,
    );

    texture_id
}
