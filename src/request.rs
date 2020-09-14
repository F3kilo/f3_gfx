use crate::backend::{Backend, TexData, TexFuture, TexId};
use crate::loading_tex::LoadingTex;
use crate::tex_unloader::TexUnloader;
use log::error;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use tokio::runtime::Runtime;

pub struct UnloadTexRequest {
    tex_id: TexId,
}

impl UnloadTexRequest {
    pub fn new(tex_id: TexId) -> Self {
        Self { tex_id }
    }

    pub fn id(&self) -> TexId {
        self.tex_id
    }
}

pub struct LoadTexRequest {
    path: PathBuf,
    response: LoadTexResponse,
}

pub struct LoadTexResponse {
    sender: Sender<LoadingTex>,
}

impl LoadTexResponse {
    pub fn new(sender: Sender<LoadingTex>) -> Self {
        Self { sender }
    }

    pub fn send(&self, loading_tex: LoadingTex) {
        self.sender.send(loading_tex).unwrap_or_else(|_| {
            error!("Try to send loading texture to requester, but it is dropped.")
        });
    }
}

pub enum Request {
    LoadTex(LoadTexRequest),
    UnloadTex(UnloadTexRequest),
}

impl From<LoadTexRequest> for (LoadTexResponse, PathBuf) {
    fn from(request: LoadTexRequest) -> Self {
        (request.response, request.path)
    }
}

impl From<UnloadTexRequest> for Request {
    fn from(request: UnloadTexRequest) -> Self {
        Request::UnloadTex(request)
    }
}

pub struct RequestProcessor {
    sender: Sender<Request>,
    receiver: Receiver<Request>,
    rt: Arc<Runtime>,
}

impl RequestProcessor {
    pub fn new(rt: Arc<Runtime>) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender,
            receiver,
            rt,
        }
    }

    pub fn process(&mut self, backend: &mut Box<dyn Backend>) {
        let requests: Vec<Request> = self.receiver.try_iter().collect();
        for request in requests {
            match request {
                Request::LoadTex(load_tex) => self.load_texture(load_tex, backend),
                Request::UnloadTex(unload_tex) => backend.remove_tex(unload_tex.id()),
            }
        }
    }

    pub fn tex_unloader(&self) -> TexUnloader {
        TexUnloader::new(self.sender.clone())
    }

    fn load_texture(&mut self, load_tex_request: LoadTexRequest, backend: &mut Box<dyn Backend>) {
        let (response, path) = load_tex_request.into();
        response.send(LoadingTex::new(
            path,
            backend,
            self.rt.clone(),
            self.tex_unloader(),
        ));
    }
}
