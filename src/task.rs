use crate::loading_tex::TexLoadResult;
use crate::tex::{Tex, TexId};
use log::warn;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;

pub async fn load_and_send(
    path: PathBuf,
    tex_reader: TexReader,
    tex_writer: TexWriter,
    sender: Sender<TexLoadResult>,
) {
    let result = load(path, tex_reader, tex_writer).await;
    send_result(result, sender).await;
}

pub async fn load(path: PathBuf, tex_reader: TexReader, tex_writer: TexWriter) -> TexLoadResult {
    let tex_data = tex_reader.read(&path).await?;
    tex_writer.write(tex_data).await.map_err(|e| e.into())
}

async fn send_result(result: TexLoadResult, sender: Sender<TexLoadResult>) {
    sender.send(result).unwrap_or_else(|_| {
        warn!("Try to send response to LoadTextureRequest, but sender is disconnected.")
    });
}

pub struct TexData {}
pub struct TexWriter {}

impl TexWriter {
    pub async fn write(&self, data: TexData) -> WriteResult {
        todo!()
    }
}

pub type WriteResult = Result<Tex, WriteError>;

pub struct WriteError;

pub struct TexReader {}

impl TexReader {
    pub async fn read(&self, path: &Path) -> ReadResult {
        todo!()
    }
}

pub type ReadResult = Result<TexData, ReadError>;
pub struct ReadError;

pub async fn unload_tex(remover: TexRemover, id: TexId) {
    remover.remove(id).await;
}

pub struct TexRemover {}

impl TexRemover {
    pub async fn remove(&self, id: TexId) {
        todo!()
    }
}
