use std::future::Future;
use std::pin::Pin;
use std::task::Context;

pub struct TexData {} // todo: fill with data

pub trait Backend: Send {
    fn add_tex(&mut self, data: TexData) -> TexFuture;
    fn remove_tex(&mut self, id: TexId);
}

pub type TexAddResult = Result<TexId, AddError>;
pub type TexFuture = Pin<Box<dyn Future<Output = TexAddResult> + Send>>;

pub struct AddError {} // todo: impl Error

pub fn poll_future(future: &mut TexFuture) -> std::task::Poll<TexAddResult> {
    let mut context = Context::from_waker(futures_util::task::noop_waker_ref());
    Pin::poll(Pin::new(future), &mut context)
}

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct TexId(u64);
