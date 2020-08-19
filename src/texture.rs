use crate::back::error::LoadError;
use crate::back::man_tex::TextureId;
use crate::error::NotFoundError;
use crate::managers::tex::TextureManager;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;

pub struct UniqueTexture {
    id: TextureId,
    manager: TextureManager,
}

impl UniqueTexture {
    pub fn load(path: PathBuf, mut manager: TextureManager) -> Result<Self, LoadError> {
        let id = manager.load_texture(path)?;
        Ok(Self { manager, id })
    }

    pub fn from_raw(id: TextureId, manager: TextureManager) -> Result<Self, NotFoundError> {
        if !manager.contains(id) {
            return Err(NotFoundError::default());
        }
        Ok(Self { id, manager })
    }

    pub fn get_id(&self) -> TextureId {
        self.id
    }
}

impl Drop for UniqueTexture {
    fn drop(&mut self) {
        self.manager.drop_texture(self.id);
    }
}

impl fmt::Debug for UniqueTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture #{:?}", self.id)
    }
}

impl PartialEq for UniqueTexture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for UniqueTexture {}

impl Hash for UniqueTexture {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Texture {
    unique: Arc<UniqueTexture>,
}

impl Texture {
    pub fn load(path: PathBuf, manager: TextureManager) -> Result<Self, LoadError> {
        UniqueTexture::load(path, manager).map(|unique| Self {
            unique: Arc::new(unique),
        })
    }

    pub fn get_id(&self) -> TextureId {
        self.unique.get_id()
    }

    pub fn from_raw(id: TextureId, manager: TextureManager) -> Result<Self, NotFoundError> {
        UniqueTexture::from_raw(id, manager).map(|unique| Self {
            unique: Arc::new(unique),
        })
    }
}
