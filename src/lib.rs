mod loading_tex;
mod resource_loader;
mod task;
mod tasker;
mod tex;

use crate::resource_loader::ResourceLoader;
use crate::tasker::Tasker;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct Graphics {
    tasker: Tasker,
}

impl Graphics {
    pub fn new(rt: Arc<Runtime>) -> Self {
        let tasker = Tasker::new(rt);
        Self { tasker }
    }

    pub fn get_resource_loader(&self) -> ResourceLoader {
        ResourceLoader::new(self.tasker.get_sender())
    }
}
