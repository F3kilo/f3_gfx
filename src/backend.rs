use std::future::Future;
use std::pin::Pin;
use std::task::Context;

pub struct TexData {}

pub trait Backend {
    fn load_tex(&mut self, data: TexData) -> TexFuture;
}

pub type TexLoadResult = Result<TexId, LoadError>;
pub type TexFuture = Pin<Box<dyn Future<Output = TexLoadResult>>>;

pub enum LoadStatus {
    Ready(TexLoadResult),
    Loading,
    Taken,
}

pub struct LoadError {}

pub fn poll_future(future: &mut TexFuture) -> std::task::Poll<TexLoadResult> {
    let mut context = Context::from_waker(futures_util::task::noop_waker_ref());
    Pin::poll(Pin::new(future), &mut context)
}

pub struct TexId(u64);
