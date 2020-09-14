use crate::backend;
use crate::backend::{AddError, Backend, TexAddResult, TexData, TexId};
use crate::tex_unloader::TexUnloader;
use futures_util::core_reexport::task::Poll;
use futures_util::core_reexport::time::Duration;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub struct LoadingTex {
    state: LoadingTexState,
    unloader: TexUnloader,
    rt: Arc<Runtime>,
}

impl LoadingTex {
    pub fn new(
        path: PathBuf,
        backend: &mut Box<dyn Backend>,
        rt: Arc<Runtime>,
        unloader: TexUnloader,
    ) -> Self {
        let handle = rt.spawn(Self::load_tex_async(path, backend));

        Self {
            state: LoadingTexState::Loading(handle),
            unloader,
            rt,
        }
    }

    pub fn status(&mut self) -> TexLoadStatus {
        let load_status = match &mut self.state {
            LoadingTexState::Loading(fut) => match backend::poll_future(fut) {
                Poll::Ready(load_result) => TexLoadStatus::Ready(load_result),
                Poll::Pending => TexLoadStatus::Loading,
            },
            LoadingTexState::Taken => TexLoadStatus::Taken,
        };

        if let TexLoadStatus::Ready(_) = &load_status {
            self.state = LoadingTexState::Taken
        };
        load_status
    }

    pub fn wait_ready(&mut self) -> Option<TexAddResult> {
        loop {
            let status = self.status();
            match status {
                TexLoadStatus::Ready(load_result) => return Some(load_result),
                TexLoadStatus::Taken => return None,
                TexLoadStatus::Loading => std::thread::sleep(Duration::from_millis(16)),
            }
        }
    }

    async fn load_tex_async(path: PathBuf, back: &mut Box<dyn Backend>) -> TexLoadResult {
        let tex_data = Self::read_tex_file(path).await?;
        back.add_tex(tex_data).await.map_err(|e| e.into())
    }

    async fn read_tex_file(path: PathBuf) -> ReadTexResult {
        Ok(TexData {}) // todo: actually read file
    }
}

impl Drop for LoadingTex {
    fn drop(&mut self) {
        if let Some(TexAddResult::Ok(id)) = self.wait_ready() {
            self.unloader.unload(id)
        }
    }
}

enum LoadingTexState {
    Loading(JoinHandle<TexLoadResult>),
    Taken,
}

pub enum TexLoadStatus {
    Ready(TexLoadResult),
    Loading,
    Taken,
}

type LoadResult<T> = Result<T, LoadError>;
type TexLoadResult = LoadResult<TexId>;

enum LoadError {
    ReadError(ReadError),
    AddError(AddError),
}

impl From<ReadError> for LoadError {
    fn from(e: ReadError) -> Self {
        LoadError::ReadError(e)
    }
}

impl From<AddError> for LoadError {
    fn from(e: AddError) -> Self {
        LoadError::AddError(e)
    }
}

type ReadResult<T> = Result<T, ReadError>;
type ReadTexResult = ReadResult<TexData>;

enum ReadError {
    OpenError,
    ParseError,
}
