use crate::backend;
use crate::backend::{LoadStatus, TexFuture, TexLoadResult};
use crate::tex_unloader::TexUnloader;
use futures_util::core_reexport::task::Poll;
use futures_util::core_reexport::time::Duration;

pub struct LoadingTex {
    state: LoadingTexState,
    unloader: TexUnloader,
}

impl LoadingTex {
    pub fn new(tex_future: TexFuture, unloader: TexUnloader) -> Self {
        Self {
            state: LoadingTexState::Loading(tex_future),
            unloader,
        }
    }

    pub fn status(&mut self) -> LoadStatus {
        let load_status = match &mut self.state {
            LoadingTexState::Loading(fut) => match backend::poll_future(fut) {
                Poll::Ready(load_result) => LoadStatus::Ready(load_result),
                Poll::Pending => LoadStatus::Loading,
            },
            LoadingTexState::Taken => LoadStatus::Taken,
        };

        if let LoadStatus::Ready(_) = &load_status {
            self.state = LoadingTexState::Taken
        };
        load_status
    }

    pub fn wait_ready(&mut self) -> Option<TexLoadResult> {
        loop {
            let status = self.status();
            match status {
                LoadStatus::Ready(load_result) => return Some(load_result),
                LoadStatus::Taken => return None,
                LoadStatus::Loading => std::thread::sleep(Duration::from_millis(16)),
            }
        }
    }
}

impl Drop for LoadingTex {
    fn drop(&mut self) {
        if let Some(TexLoadResult::Ok(id)) = self.wait_ready() {
            self.unloader.unload(id)
        }
    }
}

enum LoadingTexState {
    Loading(TexFuture),
    Taken,
}
