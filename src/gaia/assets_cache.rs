use crate::gaia::mesh::Texture;
use crate::gaia::model::Model;
use crate::gaia::utils::load_texture_from_dir;
use std::collections::HashMap;
use std::path::Path;
use log::{info, trace, warn};

#[derive(Default)]
pub struct AssetsCache {
    textures: HashMap<String, Texture>,
    models: HashMap<String, Model>,
}

impl AssetsCache {
    pub fn get_model(&mut self, path: &str) -> Model {
        match self.models.get(path) {
            Some(model) => model.clone(),
            None => {
                info!("Loading model: {}", path);
                let model = Model::new(path, self);
                self.models.insert(path.to_string(), model.clone());
                model
            }
        }
    }

    pub fn get_model_ext(&mut self, path: &str, diff_texture: Option<&str>) -> Model {
        match self.models.get(path) {
            Some(model) => model.clone(),
            None => {
                info!("Loading model: {}", path);
                let model = Model::new_ext(path, diff_texture, self);
                self.models.insert(path.to_string(), model.clone());
                model
            }
        }
    }

    pub fn get_material_texture(&mut self, dir: &str, path: &str, type_name: &str) -> Texture {
        match self.textures.get(path) {
            Some(texture) => texture.clone(),
            None => {
                let directory: String = dir.into();
                let texture = Texture {
                    id: unsafe { load_texture_from_dir(path, &directory) },
                    type_: type_name.into(),
                    path: path.into(),
                };
                self.textures.insert(path.to_string(), texture.clone());
                texture
            }
        }
    }
}
