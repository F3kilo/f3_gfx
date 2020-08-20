use f3_gfx::back::error::LoadError;
use f3_gfx::back::man_geom::{GeometryId, ManageGeometries};
use slog::Logger;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static GEOMETRY_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct FirstBackGeometryManager {
    geometries: HashSet<GeometryId>,
    logger: Logger,
}

impl FirstBackGeometryManager {
    pub fn new(logger: Logger) -> FirstBackGeometryManager {
        Self {
            geometries: HashSet::new(),
            logger,
        }
    }
}

impl ManageGeometries for FirstBackGeometryManager {
    fn load_geometry(&mut self, _path: PathBuf) -> Result<GeometryId, LoadError> {
        let id = GEOMETRY_ID_COUNTER.fetch_add(1, Ordering::Relaxed).into();
        info!(self.logger, "FirstBackGeometryManager loads: {:?}", id);
        self.geometries.insert(id);
        Ok(id)
    }

    fn drop_geometry(&mut self, id: GeometryId) -> bool {
        info!(self.logger, "FirstBackGeometryManager drops: {:?}", id);
        self.geometries.remove(&id)
    }

    fn contains(&self, id: GeometryId) -> bool {
        info!(
            self.logger,
            "FirstBackGeometryManager checks containing: {:?}", id
        );
        self.geometries.contains(&id)
    }

    fn ids(&self) -> Vec<GeometryId> {
        info!(self.logger, "FirstBackGeometryManager return ids:");
        self.geometries.iter().cloned().collect()
    }
}
