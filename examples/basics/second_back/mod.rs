pub mod tex_man;

use crate::first_back::tex_man::FirstBackTextureManager;
use crate::second_back::tex_man::SecondBackTextureManager;
use f3_gfx::back::man_tex::ManageTextures;
use f3_gfx::back::GraphicsBackend;
use slog::Logger;

pub struct SecondBack {
    tex_manager: SecondBackTextureManager,
    logger: Logger,
}

impl SecondBack {
    pub fn new(logger: Logger) -> Self {
        Self {
            tex_manager: SecondBackTextureManager::new(logger.clone()),
            logger,
        }
    }
}

impl GraphicsBackend for SecondBack {
    fn get_mut_texture_manager(&mut self) -> &mut dyn ManageTextures {
        &mut self.tex_manager
    }

    fn get_texture_manager(&self) -> &dyn ManageTextures {
        &self.tex_manager
    }
}
