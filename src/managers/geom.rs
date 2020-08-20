use crate::back::error::LoadError;
use crate::back::man_geom::GeometryId;
use crate::back::GraphicsBackend;
use slog::Logger;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct GeometryManager {
    backend: Arc<Mutex<Box<dyn GraphicsBackend>>>,
    logger: Logger,
}

impl GeometryManager {
    pub fn new(backend: Arc<Mutex<Box<dyn GraphicsBackend>>>, logger: Logger) -> Self {
        Self { backend, logger }
    }

    pub fn load_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError> {
        self.get_mut_backend()
            .get_mut_geometry_manager()
            .load_geometry(path)
    }

    pub fn drop_geometry(&mut self, id: GeometryId) -> bool {
        self.get_mut_backend()
            .get_mut_geometry_manager()
            .drop_geometry(id)
    }

    pub fn contains(&self, id: GeometryId) -> bool {
        self.get_mut_backend()
            .get_mut_geometry_manager()
            .contains(id)
    }

    pub fn ids(&self) -> Vec<GeometryId> {
        self.get_mut_backend().get_mut_geometry_manager().ids()
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
