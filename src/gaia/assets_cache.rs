use crate::gaia::mesh::Texture;
use crate::gaia::utils::load_texture_from_dir;
use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
pub struct AssetsCache {
    textures: HashMap<String, Texture>,
}

impl AssetsCache {
    pub fn load_material_texture(&mut self, dir: &str, path: &str, type_name: &str) -> Texture {
        match self.textures.get(path) {
            Some(texture) => texture.clone(),
            None => {
                let directory: String = dir.into();
                println!("{}", directory);
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
