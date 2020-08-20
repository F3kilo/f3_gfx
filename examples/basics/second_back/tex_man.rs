use f3_gfx::back::error::LoadError;
use f3_gfx::back::man_tex::{ManageTextures, TextureId};
use slog::Logger;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static TEXTURE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct SecondBackTextureManager {
    textures: HashSet<TextureId>,
    logger: Logger,
}

impl SecondBackTextureManager {
    pub fn new(logger: Logger) -> SecondBackTextureManager {
        Self {
            textures: HashSet::new(),
            logger,
        }
    }
}

impl ManageTextures for SecondBackTextureManager {
    fn load_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError> {
        let id = TEXTURE_ID_COUNTER.fetch_add(1, Ordering::Relaxed).into();
        info!(self.logger, "SecondBackTextureManager loads: {:?}", id);
        self.textures.insert(id);
        Ok(id)
    }

    fn drop_texture(&mut self, id: TextureId) -> bool {
        info!(self.logger, "SecondBackTextureManager drops: {:?}", id);
        self.textures.remove(&id)
    }

    fn contains(&self, id: TextureId) -> bool {
        info!(
            self.logger,
            "SecondBackTextureManager checks containing: {:?}", id
        );
        self.textures.contains(&id)
    }

    fn ids(&self) -> Vec<TextureId> {
        info!(self.logger, "SecondBackTextureManager return ids:");
        self.textures.iter().cloned().collect()
    }
}
