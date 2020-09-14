use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use tokio::runtime::Runtime;

pub type Task = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

pub struct Tasker {
    rt: Arc<Runtime>,
    sender: Sender<Task>,
    receiver: Receiver<Task>,
}

impl Tasker {
    pub fn new(rt: Arc<Runtime>) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            rt,
            sender,
            receiver,
        }
    }

    pub fn get_sender(&self) -> Sender<Task> {
        self.sender.clone()
    }

    pub fn start_tasks(&self) {
        for task in self.receiver.try_iter() {
            self.rt.spawn(task);
        }
    }
}
