#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::gaia::assets_cache::AssetsCache;
use crate::gaia::mesh::{Mesh, Texture, Vertex};
use crate::gaia::shader::Shader;
use crate::gaia::utils::*;
use cgmath::{vec2, vec3};
use std::path::Path;
use tobj;

#[derive(Clone)]
pub struct Model {
    /*  Model Data */
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>, // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    directory: String,
}

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new_ext(path: &str, diff_texture: Option<&str>, cache: &mut AssetsCache) -> Model {
        let pathObj = Path::new(path);
        let mut model = Model {
            meshes: Vec::<Mesh>::new(),
            textures_loaded: Vec::<Texture>::new(),
            directory: pathObj
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .to_str()
                .unwrap()
                .into(),
        };
        model.load_model(path, diff_texture, cache);
        model
    }

    pub fn new(path: &str, cache: &mut AssetsCache) -> Model {
        Model::new_ext(path, None, cache)
    }

    pub fn Draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe {
                mesh.Draw(shader);
            }
        }
    }

    // loads a model from file and stores the resulting meshes in the meshes vector.
    fn load_model(&mut self, path: &str, diffuse_path: Option<&str>, cache: &mut AssetsCache) {
        let path = Path::new(path);
        println!("Started loading model from path: {}", path.display());

        // retrieve the directory path of the filepath
        self.directory = path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap()
            .into();
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
                    position: vec3(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]),
                    normal: vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2]),
                    text_coords: vec2(t[i * 2], t[i * 2 + 1]),
                    ..Vertex::default()
                })
            }

            // process material
            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                // 1. diffuse map
                if !material.diffuse_texture.is_empty() {
                    let texture = cache.get_material_texture(
                        &self.directory,
                        &material.diffuse_texture,
                        "texture_diffuse",
                    );
                    textures.push(texture);
                }
                // 2. specular map
                if !material.specular_texture.is_empty() {
                    let texture = cache.get_material_texture(
                        &self.directory,
                        &material.specular_texture,
                        "texture_specular",
                    );
                    textures.push(texture);
                }
                // 3. normal map
                if !material.normal_texture.is_empty() {
                    let texture = cache.get_material_texture(
                        &self.directory,
                        &material.normal_texture,
                        "texture_normal",
                    );
                    textures.push(texture);
                }
            // NOTE: no height maps
            } else if diffuse_path.is_some() {
                println!("Loading {}", &diffuse_path.unwrap());
                let texture = cache.get_material_texture(
                    &self.directory,
                    &diffuse_path.unwrap(),
                    "texture_diffuse",
                );
                textures.push(texture);
            } else {
                println!("There are no materials for: {}", path.display());
            }

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
        println!("Finished loading model from path: {}", path.display());
    }
}
