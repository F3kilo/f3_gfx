mod first_back;
mod second_back;

use f3_gfx::Graphics;
#[macro_use]
extern crate slog;
use crate::first_back::FirstBack;
use crate::second_back::SecondBack;
use f3_gfx::back::GraphicsBackend;
use f3_gfx::texture::Texture;
use slog::{Drain, Logger};

fn main() {
    let logger = init_logger();
    let back = init_first_backend(logger.clone());
    let mut graphics = Graphics::new(back, Some(logger.clone()));

    use_graphics(&graphics);

    graphics.replace_backend(init_second_backend(logger.clone()));
    use_graphics(&graphics)
}

fn init_logger() -> Logger {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!())
}

fn init_first_backend(logger: Logger) -> Box<dyn GraphicsBackend> {
    Box::new(FirstBack::new(logger))
}

fn init_second_backend(logger: Logger) -> Box<dyn GraphicsBackend> {
    Box::new(SecondBack::new(logger))
}

fn use_graphics(graphics: &Graphics) {
    let tex1 = Texture::load("First tex path".into(), graphics.get_texture_manager()).unwrap();
    let tex2 = Texture::load("Second tex path".into(), graphics.get_texture_manager()).unwrap();
}
