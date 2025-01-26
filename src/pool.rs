use std::thread::{self, JoinHandle};

pub trait ThreadPool {
    fn spawn<F, T>(&self, action: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static;
}

pub struct UnboundedThreadPool;

impl ThreadPool for UnboundedThreadPool {
    fn spawn<F, T>(&self, action: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        thread::spawn(action)
    }
}
