mod geom_man;
pub mod tex_man;

use crate::second_back::geom_man::SecondBackGeometryManager;
use crate::second_back::tex_man::SecondBackTextureManager;
use f3_gfx::back::man_geom::ManageGeometries;
use f3_gfx::back::man_tex::ManageTextures;
use f3_gfx::back::GraphicsBackend;
use slog::Logger;

pub struct SecondBack {
    tex_manager: SecondBackTextureManager,
    geom_manager: SecondBackGeometryManager,
    _logger: Logger,
}

impl SecondBack {
    pub fn new(logger: Logger) -> Self {
        Self {
            tex_manager: SecondBackTextureManager::new(logger.clone()),
            geom_manager: SecondBackGeometryManager::new(logger.clone()),
            _logger: logger,
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

    fn get_mut_geometry_manager(&mut self) -> &mut dyn ManageGeometries {
        &mut self.geom_manager
    }

    fn get_geometry_manager(&self) -> &dyn ManageGeometries {
        &self.geom_manager
    }
}
