use std::sync::Arc;

use crate::pool::ThreadPool;

pub trait Node {
    type Output;
    fn call(self) -> Self::Output;
}

pub trait ParNode {
    type Output;
    fn call_par(self, pool: Arc<impl ThreadPool>) -> Self::Output;
}

impl<T: Node> ParNode for T {
    type Output = T::Output;
    fn call_par(self, _: Arc<impl ThreadPool>) -> Self::Output {
        self.call()
    }
}
