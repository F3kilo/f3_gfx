use crate::loading_tex::TexLoadResult;
use async_trait::async_trait;
use log::warn;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

#[async_trait]
pub trait Task {
    async fn perform(&mut self);
}

pub struct LoadTextureTask {
    path: PathBuf,
    tex_reader: TexReader,
    tex_writer: TexWriter,
    sender: Sender<TexLoadResult>,
}

impl LoadTextureTask {
    pub fn new(
        path: PathBuf,
        reader: TexReader,
        writer: TexWriter,
        sender: Sender<TexLoadResult>,
    ) -> Self {
        Self {
            path,
            tex_reader: reader,
            tex_writer: writer,
            sender,
        }
    }

    fn send_result(&self, result: TexLoadResult) {
        self.sender.send(result).unwrap_or_else(|e| {
            warn!("Try to send response to LoadTextureRequest, but sender is disconnected.")
        });
    }

    async fn load(&mut self) -> TexLoadResult {
        let tex_data = self.tex_reader.read(&self.path).await?;
        self.tex_writer.write(tex_data).await
    }
}

struct TexData {}
struct TexWriter {}

impl TexWriter {
    pub async fn write(&self, data: TexData) -> WriteResult {}
}

struct TexReader {}

#[async_trait]
impl Task for LoadTextureTask {
    async fn perform(&mut self) {
        self.send_result(Self::load(&mut self).await);
    }
}
