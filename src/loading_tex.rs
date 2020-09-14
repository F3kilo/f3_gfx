use crate::resource_loader::{LoadResult, TexUnloader};
use crate::tex::Tex;
use log::warn;
use std::sync::mpsc::{Receiver, TryRecvError};

pub type TexLoadResult = LoadResult<Tex>;

pub struct LoadingTex {
    recv: Option<Receiver<TexLoadResult>>,
    unloader: TexUnloader,
}

impl LoadingTex {
    pub fn new(recv: Receiver<LoadResult<Tex>>, unloader: TexUnloader) -> Self {
        Self {
            recv: Some(recv),
            unloader,
        }
    }

    pub fn try_take(&mut self) -> Result<TexLoadResult, TakeError> {
        if let Some(recv) = &self.recv {
            let received = recv.try_recv();
            return match received {
                Ok(tex_result) => {
                    self.recv = None;
                    Ok(tex_result)
                }
                Err(e) => match e {
                    TryRecvError::Empty => Err(TakeError::NotReady),
                    TryRecvError::Disconnected => {
                        warn!("Try take loading texture, but receiver is disconnected.");
                        Err(TakeError::NotAvailable)
                    }
                },
            };
        };

        Err(TakeError::AlreadyTaken)
    }

    pub fn wait_ready(&mut self) -> Result<TexLoadResult, TakeError> {
        if let Some(recv) = &self.recv {
            let received = recv.recv();
            return match received {
                Ok(tex_result) => {
                    self.recv = None;
                    Ok(tex_result)
                }
                Err(_) => {
                    warn!("Wait for texture loading, but receiver is disconnected.");
                    Err(TakeError::NotAvailable)
                }
            };
        };

        Err(TakeError::AlreadyTaken)
    }
}

pub enum TakeError {
    NotReady,
    NotAvailable,
    AlreadyTaken,
}
