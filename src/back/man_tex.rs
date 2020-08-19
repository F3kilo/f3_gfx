use crate::back::error::LoadError;
use std::path::PathBuf;

pub trait ManageTextures {
    fn load_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError>;
    fn drop_texture(&mut self, id: TextureId) -> bool;

    fn contains(&self, id: TextureId) -> bool;
    fn ids(&self) -> Vec<TextureId>;
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct TextureId(u64);

impl From<u64> for TextureId {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
