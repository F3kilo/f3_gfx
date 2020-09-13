use crate::backend::Backend;
use crate::request::RequestProcessor;

mod backend;
mod loading_tex;
mod request;
mod tex_unloader;

pub struct Graphics {
    back: Box<dyn Backend>,
    requests: RequestProcessor,
}

impl Graphics {
    pub fn new(back: Box<dyn Backend>) -> Self {
        Self {
            back,
            requests: RequestProcessor::new(),
        }
    }

    pub fn process_requests(&mut self) {
        self.requests.process(&mut self.back);
    }
}
