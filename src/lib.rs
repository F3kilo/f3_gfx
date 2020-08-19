pub mod back;

#[macro_use]
extern crate slog;

use crate::back::GraphicsBackend;
use slog::Logger;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Graphics {
    logger: Logger,
    backend: Arc<Mutex<Box<dyn GraphicsBackend>>>,
}

impl Graphics {
    pub fn new(backend: Box<dyn GraphicsBackend>, logger: Option<Logger>) -> Self {
        let backend = Arc::new(Mutex::new(backend));
        let logger = logger.unwrap_or_else(|| Logger::root(slog::Discard, o!()));
        Self { backend, logger }
    }

    pub fn replace_backend(
        &mut self,
        new_backend: Box<dyn GraphicsBackend>,
    ) -> Box<dyn GraphicsBackend> {
        let mut mut_back = self.backend.lock().unwrap();
        std::mem::replace(&mut mut_back, new_backend)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
