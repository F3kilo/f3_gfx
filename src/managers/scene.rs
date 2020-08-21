use crate::back::error::LoadError;
use crate::back::man_scene::SceneId;
use crate::back::GraphicsBackend;
use slog::Logger;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct SceneManager {
    backend: Arc<Mutex<Box<dyn GraphicsBackend>>>,
    logger: Logger,
}

impl SceneManager {
    pub fn new(backend: Arc<Mutex<Box<dyn GraphicsBackend>>>, logger: Logger) -> Self {
        Self { backend, logger }
    }

    pub fn create_scene(&mut self, path: PathBuf) -> Result<SceneId, LoadError> {
        self.get_mut_backend()
            .get_mut_scene_manager()
            .create_scene(path)
    }

    pub fn drop_scene(&mut self, id: SceneId) -> bool {
        self.get_mut_backend()
            .get_mut_scene_manager()
            .drop_scene(id)
    }

    pub fn contains(&self, id: SceneId) -> bool {
        self.get_mut_backend().get_mut_scene_manager().contains(id)
    }

    pub fn ids(&self) -> Vec<SceneId> {
        self.get_mut_backend().get_mut_scene_manager().ids()
    }

    fn get_mut_backend(&self) -> MutexGuard<Box<dyn GraphicsBackend>> {
        self.backend.lock().unwrap_or_else(|_| {
            crit!(
                self.logger,
                "Mutex to graphics backend poisoned. Aborting..."
            );
            panic!("Mutex to graphics backend poisoned.");
        })
    }
}
