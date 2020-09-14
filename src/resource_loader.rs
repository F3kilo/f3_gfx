use crate::loading_tex::LoadingTex;
use crate::task;
use crate::task::{ReadError, TexReader, TexRemover, TexWriter, WriteError};
use crate::tasker::Task;
use crate::tex::TexId;
use log::warn;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct ResourceLoader {
    task_sender: Sender<Task>,
}

impl ResourceLoader {
    pub fn new(task_sender: Sender<Task>) -> Self {
        Self { task_sender }
    }

    pub fn load_tex(&self, path: PathBuf) -> LoadingTex {
        let (sender, receiver) = mpsc::channel();
        let task = Box::pin(task::load_and_send(
            path,
            self.get_tex_reader(),
            self.get_tex_writer(),
            sender,
        ));
        self.task_sender.send(task).unwrap_or_else(|_| {
            warn!("Try to send LoadTextureRequest, but sender is disconnected.")
        });
        LoadingTex::new(receiver, self.get_tex_unloader())
    }

    fn get_tex_unloader(&self) -> TexUnloader {
        TexUnloader::new(self.task_sender.clone())
    }

    fn get_tex_reader(&self) -> TexReader {
        todo!()
    }

    fn get_tex_writer(&self) -> TexWriter {
        todo!()
    }
}

pub struct TexUnloader {
    task_sender: Sender<Task>,
}

impl TexUnloader {
    pub fn new(task_sender: Sender<Task>) -> Self {
        Self { task_sender }
    }

    pub fn unload(&self, id: TexId) {
        let task = Box::pin(async {});
        self.task_sender
            .send(task)
            .unwrap_or_else(|_| warn!("Try to unload texture {:?}, but can't send request.", id));
    }

    fn get_tex_remover(&self) -> TexRemover {
        todo!()
    }
}

pub type LoadResult<T> = Result<T, LoadError>;

pub enum LoadError {
    ReadError(ReadError),
    WriteError(WriteError),
}

impl From<ReadError> for LoadError {
    fn from(e: ReadError) -> Self {
        Self::ReadError(e)
    }
}

impl From<WriteError> for LoadError {
    fn from(e: WriteError) -> Self {
        Self::WriteError(e)
    }
}
