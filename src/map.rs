use crate::assets_cache::AssetsCache;
use crate::camera::*;
use crate::components::*;
use crate::consts;
use crate::light::*;
use crate::math::{perspective, vec3, Deg, Matrix4, Point3};
use crate::sky::Sky;
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MapObject {
    pub model_hash: u64,
    pub transform: Transform,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapSave {
    pub objects: Vec<MapObject>,
    pub camera: Camera,
}

impl MapSave {
    pub fn save(map: &Map, _path: &str) {
        let mut objects = Vec::with_capacity(map.models.len());
        for m in &map.models {
            objects.push(MapObject {
                model_hash: m.hash,
                transform: m.transform,
            });
        }
        let ms = MapSave {
            camera: map.camera.clone(),
            objects: objects,
        };
        match serde_yaml::to_string(&ms) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{:?}", e),
        }
    }

    pub fn load(path: &str, cache: &mut AssetsCache) -> Map {
        let mut map = Map::default();
        info!("Loading map from file: {}", path);

        use std::fs;
        let savefile = fs::read_to_string(path);
        if savefile.is_err() {
            error!("Cannot read file {}: {:?}", path, savefile);
            return map;
        }

        let save: Result<MapSave, serde_yaml::Error> = serde_yaml::from_str(&savefile.unwrap());

        if save.is_err() {
            error!("Cannot parse file {}: {:?}", path, save);
            return map;
        }

        for m in &save.unwrap().objects {
            let model = cache.get_model_by_hash(&m.model_hash);
            if model.is_none() {
                continue;
            }
            map.models.push(ModelComponent {
                transform: m.transform,
                hash: m.model_hash,
                model: model.unwrap(),
            })
        }
        info!("Map loaded: {}", path);

        map
    }
}

pub struct Map {
    pub models: Vec<ModelComponent>,
    pub camera: Camera,
    pub lighting_system: LightingSystem,
    pub sky: Sky,
}

impl Default for Map {
    fn default() -> Self {
        let sky = unsafe { Sky::new() };
        Map {
            models: Vec::new(),
            camera: Camera {
                position: Point3::new(0.0, 8.0, 13.0),
                front: vec3(0.0, -0.4, -1.0),
                up: vec3(0.0, 1.0, -0.4),
                right: vec3(1.0, 0.0, 0.0),
                yaw: -90.0,
                pitch: -20.0,
                ..Camera::default()
            },
            lighting_system: LightingSystem::default(),
            sky: sky,
        }
    }
}

impl Map {
    pub unsafe fn draw(&mut self) {
        use crate::math::prelude::*;
        // view/projection transformations
        let projection: Matrix4<f32> = perspective(
            Deg(self.camera.zoom),
            consts::SCR_WIDTH as f32 / consts::SCR_HEIGHT as f32,
            0.1,
            1000.0,
        );
        let view = self.camera.get_view_matrix();
        let view_pos = self.camera.position.to_vec();
        self.lighting_system
            .prepare_for_draw(&projection, &view, &view_pos);

        for model in self.models.iter() {
            model.draw(&self.lighting_system.shader);
        }
        self.sky.draw(view, projection);
    }
}
