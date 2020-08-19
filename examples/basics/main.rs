mod first_back;

use f3_gfx::Graphics;
#[macro_use]
extern crate slog;
use crate::first_back::FirstBack;
use f3_gfx::back::GraphicsBackend;
use f3_gfx::texture::Texture;
use slog::{Drain, Logger};

fn main() {
    let logger = init_logger();
    let back = init_backend(logger.clone());
    let graphics = Graphics::new(back, Some(logger));

    let tex1 = Texture::new("First tex path".into(), graphics.get_texture_manager()).unwrap();
    let tex2 = Texture::new("Second tex path".into(), graphics.get_texture_manager()).unwrap();
}

fn init_logger() -> Logger {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!())
}

fn init_backend(logger: Logger) -> Box<dyn GraphicsBackend> {
    Box::new(FirstBack::new(logger))
}
