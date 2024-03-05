use crate::mesh::Texture;
use crate::model::Model;
use crate::utils::load_texture_from_dir;
use log::{error, info};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(Default)]
pub struct AssetsCache {
    textures: HashMap<u64, Texture>,
    models: HashMap<u64, Model>,
}

impl AssetsCache {
    pub fn load_2d(&mut self, path: &str) -> Model {
        let fullpath = Path::new(&path);
        let dir = fullpath.parent().unwrap().to_str().unwrap().to_string();
        let filename = fullpath.file_name().unwrap().to_str().unwrap();
        let texture = self.get_material_texture(&dir, filename, "texture_diffuse");
        let mut model = Model::new_2d(self);
        for ele in &mut model.meshes {
            ele.textures.push(texture.clone())
        }
        model
    }

    pub fn load_all_from_file(&mut self, path: &str) {
        let path = std::path::Path::new(path);
        let meta = std::fs::metadata(path);
        if meta.is_err() {
            error!("There is no assets file");
            return;
        }
        use std::fs;
        use std::io::prelude::*;
        use std::io::BufReader;
        let buf = BufReader::new(fs::File::open(path).expect("no such file"));
        let lines: Vec<String> = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        for line in lines {
            let mut splitted = line.split_whitespace();
            let path = splitted.next();
            if path.is_none() || self.has_model(path.unwrap()) {
                error!("Skip wrong line: {}", line);
                continue;
            }
            let texture = splitted.next();
            self.load_model_ext(path.unwrap(), texture);
            info!("Added model from path {}", path.unwrap());
        }
    }

    pub fn has_model(&self, path: &str) -> bool {
        self.models.contains_key(&Self::path_hash(path))
    }

    pub fn get_model(&mut self, path: &str) -> Model {
        self.get_model_ext(path, None)
    }

    pub fn get_model_ext(&mut self, path: &str, diff_texture: Option<&str>) -> Model {
        match self.models.get(&Self::path_hash(path)) {
            Some(model) => model.clone(),
            None => {
                self.load_model_ext(path, diff_texture);
                self.get_model_ext(path, diff_texture)
            }
        }
    }

    pub fn get_model_by_hash(&mut self, hash: &u64) -> Option<Model> {
        self.models.get(hash).cloned()
    }

    fn load_model_ext(&mut self, path: &str, diff_texture: Option<&str>) {
        let hash = Self::path_hash(path);
        info!("Loading model: {}({})", path, hash);
        let model = Model::new_ext(path, diff_texture, self, false);
        self.models.insert(hash, model);
    }

    pub fn get_material_texture(&mut self, dir: &str, path: &str, type_name: &str) -> Texture {
        match self.textures.get(&Self::path_hash(path)) {
            Some(texture) => texture.clone(),
            None => {
                let directory: String = dir.into();
                let texture = Texture {
                    id: unsafe { load_texture_from_dir(path, &directory) },
                    type_: type_name.into(),
                    path: path.into(),
                };
                self.textures.insert(Self::path_hash(path), texture.clone());
                texture
            }
        }
    }

    pub fn path_hash(path: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        hasher.finish()
    }
}
