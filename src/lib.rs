use crate::backend::Backend;
use crate::request::RequestProcessor;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod backend;
mod loading_tex;
mod request;
mod tex_unloader;

pub struct Graphics {
    back: Box<dyn Backend>,
    requests: RequestProcessor,
}

impl Graphics {
    pub fn new(back: Box<dyn Backend>, rt: Arc<Runtime>) -> Self {
        Self {
            back,
            requests: RequestProcessor::new(rt),
        }
    }

    pub fn process_requests(&mut self) {
        self.requests.process(&mut self.back);
    }
}
