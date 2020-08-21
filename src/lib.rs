pub mod back;
pub mod error;
pub mod geometry;
pub mod managers;
pub mod scene;
pub mod texture;

#[macro_use]
extern crate slog;

use crate::back::GraphicsBackend;
use crate::managers::geom::GeometryManager;
use crate::managers::scene::SceneManager;
use crate::managers::tex::TextureManager;
use slog::Logger;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Graphics {
    backend: Arc<Mutex<Box<dyn GraphicsBackend>>>,
    logger: Logger,
}

impl Graphics {
    pub fn new(backend: Box<dyn GraphicsBackend>, logger: Option<Logger>) -> Self {
        let backend = Arc::new(Mutex::new(backend));
        let logger = logger.unwrap_or_else(|| Logger::root(slog::Discard, o!()));
        Self { backend, logger }
    }

    pub fn replace_backend(
        &mut self,
        new_backend: Box<dyn GraphicsBackend>,
    ) -> Box<dyn GraphicsBackend> {
        let mut mut_back = self.backend.lock().unwrap();
        std::mem::replace(&mut mut_back, new_backend)
    }

    pub fn get_texture_manager(&self) -> TextureManager {
        TextureManager::new(self.backend.clone(), self.logger.clone())
    }

    pub fn get_geometry_manager(&self) -> GeometryManager {
        GeometryManager::new(self.backend.clone(), self.logger.clone())
    }

    pub fn get_scene_manager(&self) -> SceneManager {
        SceneManager::new(self.backend.clone(), self.logger.clone())
    }
}
