use crate::backend::{Backend, TexData, TexId};
use crate::loading_tex::LoadingTex;
use crate::tex_unloader::TexUnloader;
use log::error;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

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
    response: LoadTexResponse,
    tex_data: TexData,
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

impl From<LoadTexRequest> for (LoadTexResponse, TexData) {
    fn from(request: LoadTexRequest) -> Self {
        (request.response, request.tex_data)
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
}

impl RequestProcessor {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }

    pub fn process(&mut self, backend: &mut Box<dyn Backend>) {
        let requests: Vec<Request> = self.receiver.try_iter().collect();
        for request in requests {
            match request {
                Request::LoadTex(load_tex) => self.load_texture(load_tex, backend),
                Request::UnloadTex(unload_tex) => backend.unload_tex(unload_tex.id()),
            }
        }
    }

    pub fn tex_unloader(&self) -> TexUnloader {
        TexUnloader::new(self.sender.clone())
    }

    fn load_texture(&mut self, load_tex_request: LoadTexRequest, backend: &mut Box<dyn Backend>) {
        let (response, data) = load_tex_request.into();
        let tex_fut = backend.load_tex(data);
        response.send(LoadingTex::new(tex_fut, self.tex_unloader()));
    }
}
