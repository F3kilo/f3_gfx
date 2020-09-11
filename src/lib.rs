mod backend;
mod loading_tex;

use crate::backend::{Backend, TexData};
use crate::loading_tex::{LoadingTex, TexUnloader};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct Graphics {
    back: Box<dyn Backend>,
    requests: RequestProcessor,
}

impl Graphics {
    pub fn new(back: Box<dyn Backend>) -> Self {
        Self {
            back,
            requests: RequestProcessor::new(),
        }
    }

    pub fn process_requests(&mut self) {
        self.requests.process(&mut self.back);
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
        self.sender.send(loading_tex);
    }
}

impl From<LoadTexRequest> for (LoadTexResponse, TexData) {
    fn from(request: LoadTexRequest) -> Self {
        (request.response, request.tex_data)
    }
}

pub enum Request {
    LoadTex(LoadTexRequest),
}

struct RequestProcessor {
    sender: Sender<Request>,
    receiver: Receiver<Request>,
}

impl RequestProcessor {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }

    pub fn process(&mut self, backend: &mut Box<dyn Backend>) {
        let requests = self.receiver.try_iter();
        for request in requests {
            match request {
                Request::LoadTex(load_tex) => self.load_texture(load_tex, backend),
            }
        }
    }

    pub fn tex_unloader() -> TexUnloader {}

    fn load_texture(&mut self, load_tex_request: LoadTexRequest, backend: &mut Box<dyn Backend>) {
        let (response, data) = load_tex_request.into();
        let tex_fut = backend.load_tex(data);
        response.send(LoadingTex::new(tex_fut, self.tex_unloader()));
    }
}
