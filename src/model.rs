#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::c_void;
use std::path::Path;

use cgmath::{vec2, vec3};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use tobj;

use crate::mesh::{ Mesh, Texture, Vertex };
use crate::shader::Shader;
use crate::engine::*;

// #[derive(Default)]
pub struct Model {
    /*  Model Data */
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>,   // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    directory: String,
}

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new(path: &str) -> Model {
        let pathObj = Path::new(path);
        let mut model = Model{
            meshes: Vec::<Mesh>::new(),
            textures_loaded: Vec::<Texture>::new(),
            directory: pathObj.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into()
        };
        model.loadModel(path);
        model
    }

    pub fn Draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe { mesh.Draw(shader); }
        }
    }

    // loads a model from file and stores the resulting meshes in the meshes vector.
    fn loadModel(&mut self, path: &str) {
        let path = Path::new(path);
        println!("Started loading model from path: {}", path.display());

        // retrieve the directory path of the filepath
        self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let obj = tobj::load_obj(path, true);

        let (models, materials) = obj.unwrap();
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // data to fill
            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    Position:  vec3(p[i*3], p[i*3+1], p[i*3+2]),
                    Normal:    vec3(n[i*3], n[i*3+1], n[i*3+2]),
                    TexCoords: vec2(t[i*2], t[i*2+1]),
                    ..Vertex::default()
                })
            }

            // process material
            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                // 1. diffuse map
                if !material.diffuse_texture.is_empty() {
                    let texture = self.loadMaterialTexture(&material.diffuse_texture, "texture_diffuse");
                    textures.push(texture);
                }
                // 2. specular map
                if !material.specular_texture.is_empty() {
                    let texture = self.loadMaterialTexture(&material.specular_texture, "texture_specular");
                    textures.push(texture);
                }
                // 3. normal map
                if !material.normal_texture.is_empty() {
                    let texture = self.loadMaterialTexture(&material.normal_texture, "texture_normal");
                    textures.push(texture);
                }
                // NOTE: no height maps
            }

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
        println!("Finished loading model from path: {}", path.display());

    }

    fn loadMaterialTexture(&mut self, path: &str, typeName: &str) -> Texture {
        {
            let texture = self.textures_loaded.iter().find(|t| t.path == path);
            if let Some(texture) = texture {
                return texture.clone();
            }
        }

        let texture = Texture {
            id: unsafe { load_texture_from_dir(path, &self.directory) },
            type_: typeName.into(),
            path: path.into()
        };
        self.textures_loaded.push(texture.clone());
        texture
    }
}

