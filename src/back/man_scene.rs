use crate::back::error::LoadError;
use std::path::PathBuf;

pub trait ManageScenes {
    fn create_scene(&mut self, path: PathBuf) -> Result<SceneId, LoadError>;
    fn drop_scene(&mut self, id: SceneId) -> bool;

    fn contains(&self, id: SceneId) -> bool;
    fn ids(&self) -> Vec<SceneId>;
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct SceneId(u64);

impl From<u64> for SceneId {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
