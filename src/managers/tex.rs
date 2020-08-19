use crate::back::error::LoadError;
use crate::back::man_tex::TextureId;
use crate::back::GraphicsBackend;
use slog::Logger;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct TextureManager {
    backend: Arc<Mutex<Box<dyn GraphicsBackend>>>,
    logger: Logger,
}

impl TextureManager {
    pub fn new(backend: Arc<Mutex<Box<dyn GraphicsBackend>>>, logger: Logger) -> Self {
        Self { backend, logger }
    }

    pub fn load_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError> {
        self.get_mut_backend()
            .get_mut_texture_manager()
            .load_texture(path)
    }

    pub fn drop_texture(&mut self, id: TextureId) -> bool {
        self.get_mut_backend()
            .get_mut_texture_manager()
            .drop_texture(id)
    }

    pub fn contains(&self, id: TextureId) -> bool {
        self.get_mut_backend()
            .get_mut_texture_manager()
            .contains(id)
    }

    pub fn ids(&self) -> Vec<TextureId> {
        self.get_mut_backend().get_mut_texture_manager().ids()
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
