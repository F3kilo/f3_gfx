use crate::back::error::LoadError;
use crate::back::man_geom::GeometryId;
use crate::managers::geom::GeometryManager;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;

pub struct UniqueGeometry {
    id: GeometryId,
    manager: GeometryManager,
}

impl UniqueGeometry {
    pub fn load(path: PathBuf, mut manager: GeometryManager) -> Result<Self, LoadError> {
        let id = manager.load_geometry(path)?;
        Ok(Self { manager, id })
    }

    pub fn get_id(&self) -> GeometryId {
        self.id
    }
}

impl Drop for UniqueGeometry {
    fn drop(&mut self) {
        self.manager.drop_geometry(self.id);
    }
}

impl fmt::Debug for UniqueGeometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Geometry #{:?}", self.id)
    }
}

impl PartialEq for UniqueGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for UniqueGeometry {}

impl Hash for UniqueGeometry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Geometry {
    unique: Arc<UniqueGeometry>,
}

impl Geometry {
    pub fn load(path: PathBuf, manager: GeometryManager) -> Result<Self, LoadError> {
        UniqueGeometry::load(path, manager).map(|unique| Self {
            unique: Arc::new(unique),
        })
    }

    pub fn get_id(&self) -> GeometryId {
        self.unique.get_id()
    }
}
