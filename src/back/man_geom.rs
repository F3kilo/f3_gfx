use crate::back::error::LoadError;
use std::path::PathBuf;

pub trait ManageGeometries {
    fn load_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError>;
    fn drop_geometry(&mut self, id: GeometryId) -> bool;

    fn contains(&self, id: GeometryId) -> bool;
    fn ids(&self) -> Vec<GeometryId>;
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct GeometryId(u64);

impl From<u64> for GeometryId {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
