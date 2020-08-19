use crate::back::man_tex::ManageTextures;

pub mod error;
pub mod man_tex;

pub trait GraphicsBackend {
    fn get_mut_texture_manager(&mut self) -> &mut dyn ManageTextures;
    fn get_texture_manager(&self) -> &dyn ManageTextures;
}
