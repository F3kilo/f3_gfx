use crate::task::Task;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use tokio::runtime::Runtime;

pub struct Tasker {
    rt: Arc<Runtime>,
    sender: Sender<Box<dyn Task>>,
    receiver: Receiver<Box<dyn Task>>,
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

    pub fn start_tasks(&self) {
        for task in self.receiver.try_iter() {
            self.rt.spawn(task);
        }
    }
}
