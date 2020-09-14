use crate::loading_tex::LoadingTex;
use crate::task::{LoadTextureTask, Task};
use crate::tex::TexId;
use log::warn;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct ResourceLoader {
    task_sender: Sender<Box<dyn Task>>,
}

impl ResourceLoader {
    pub fn new(task_sender: Sender<Box<dyn Task>>) -> Self {
        Self { task_sender }
    }

    pub fn load_tex(&self, path: PathBuf) -> LoadingTex {
        let (sender, receiver) = mpsc::channel();
        self.task_sender
            .send(LoadTextureTask::new(path, sender).into())
            .unwrap_or_else(|_| {
                warn!("Try to send LoadTextureRequest, but sender is disconnected.")
            });
        LoadingTex::new(receiver, self.get_tex_unloader())
    }

    fn get_tex_unloader(&self) -> TexUnloader {
        TexUnloader::new(self.task_sender.clone())
    }
}

pub struct TexUnloader {
    task_sender: Sender<Box<dyn Task>>,
}

impl TexUnloader {
    pub fn new(request_sender: Sender<Box<dyn Task>>) -> Self {
        Self {
            task_sender: request_sender,
        }
    }

    pub fn unload(&self, id: TexId) {
        self.task_sender
            .send(Task::UnloadTexture(id))
            .unwrap_or_else(|_| warn!("Try to unload texture {:?}, but can't send request.", id));
    }
}

pub type LoadResult<T> = Result<T, LoadError>;

pub enum LoadError {}
