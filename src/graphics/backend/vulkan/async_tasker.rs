use std::fmt::Debug;
use std::sync::mpsc;
use std::sync::mpsc::{SendError, Sender};
use std::thread::JoinHandle;

pub trait AsyncTask: Send + Debug {
    fn perform(&mut self);
}

#[derive(Debug)]
pub enum Command {
    Finish,
    AddTask(Box<dyn AsyncTask>),
}

#[derive(Debug, Clone)]
pub struct TaskSender {
    tx: Sender<Command>,
}

impl TaskSender {
    pub fn new(tx: Sender<Command>) -> Self {
        Self { tx }
    }

    pub fn send(&self, task: Box<dyn AsyncTask>) -> Result<(), SendError<Box<dyn AsyncTask>>> {
        self.tx.send(Command::AddTask(task)).map_err(|e| {
            if let Command::AddTask(task) = e.0 {
                SendError(task)
            } else {
                panic!("Impossible error")
            }
        })
    }
}

#[derive(Debug)]
pub struct AsyncTasker {
    thread: JoinHandle<()>,
    tx: Sender<Command>,
}

impl AsyncTasker {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let thread = std::thread::spawn(move || loop {
            let command = rx.recv().unwrap();
            match command {
                Command::Finish => {
                    return;
                }
                Command::AddTask(mut task) => {
                    task.perform();
                }
            };
        });
        Self { thread, tx }
    }

    pub fn get_task_sender(&self) -> TaskSender {
        TaskSender::new(self.tx.clone())
    }
}

impl Drop for AsyncTasker {
    fn drop(&mut self) {
        self.tx
            .send(Command::Finish)
            .expect("Async tasks thread was finished unexpectedly");
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::backend::vulkan::async_tasker::{AsyncTask, AsyncTasker};
    use std::sync::mpsc::Sender;
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[derive(Debug)]
    struct Task {
        performed: Arc<Mutex<bool>>,
    }

    impl Task {
        pub fn new(performed: Arc<Mutex<bool>>) -> Self {
            Self { performed }
        }
    }

    impl AsyncTask for Task {
        fn perform(&mut self) {
            let mut guard = self.performed.lock().unwrap();
            *guard = true;
        }
    }

    #[derive(Debug)]
    struct TaskWithAnswer {
        tx: Sender<bool>,
    }

    impl TaskWithAnswer {
        pub fn new(tx: Sender<bool>) -> Self {
            Self { tx }
        }
    }

    impl AsyncTask for TaskWithAnswer {
        fn perform(&mut self) {
            thread::sleep(Duration::from_millis(100));
            self.tx.send(true).unwrap();
        }
    }

    #[test]
    fn perform_task() {
        let tasker = AsyncTasker::new();
        let performed = Arc::new(Mutex::new(false));
        let task = Task::new(performed.clone());
        tasker.get_task_sender().send(Box::new(task)).unwrap();
        thread::sleep(Duration::from_millis(100));
        let guard = performed.lock().unwrap();
        assert!(*guard)
    }

    #[test]
    fn perform_task_with_answer() {
        let tasker = AsyncTasker::new();
        let (tx, rx) = mpsc::channel();
        let task = TaskWithAnswer::new(tx);
        tasker.get_task_sender().send(Box::new(task)).unwrap();
        let timeout = Duration::from_millis(400);
        let recv = rx.recv_timeout(timeout);
        assert!(recv.is_ok())
    }
}
