pub mod error;
pub mod man_geom;
pub mod man_tex;

use crate::back::man_geom::ManageGeometries;
use crate::back::man_tex::ManageTextures;

pub trait GraphicsBackend {
    fn get_mut_texture_manager(&mut self) -> &mut dyn ManageTextures;
    fn get_texture_manager(&self) -> &dyn ManageTextures;

    fn get_mut_geometry_manager(&mut self) -> &mut dyn ManageGeometries;
    fn get_geometry_manager(&self) -> &dyn ManageGeometries;
}
