use crate::backend::TexId;
use crate::request::{Request, UnloadTexRequest};
use log::error;
use std::sync::mpsc::Sender;

pub struct TexUnloader {
    sender: Sender<Request>,
}

impl TexUnloader {
    pub fn new(sender: Sender<Request>) -> Self {
        Self { sender }
    }

    pub fn unload(&self, id: TexId) {
        self.sender
            .send(UnloadTexRequest::new(id).into())
            .unwrap_or_else(|_| {
                error!("Try to send unload texture request, but RequestProcessor is dropped.")
            });
    }
}
