use std::{cell::Cell, future::Future};

use crate::GraphNode;

pub struct Asyncified<T> {
    value: Cell<Option<T>>,
}

impl<T> Asyncified<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Cell::new(Some(value)),
        }
    }
}

impl<T> Future for Asyncified<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.value.take() {
            Some(value) => std::task::Poll::Ready(value),
            None => panic!("cannot poll after completion"),
        }
    }
}

pub struct Asyncify<Src> {
    src: Src,
}

impl<Src: GraphNode> GraphNode for Asyncify<Src> {
    type Output = Asyncified<Src::Output>;

    fn execute(self) -> Self::Output {
        Asyncified::new(self.src.execute())
    }
}

impl<T: GraphNode> AsyncExt for T {}
pub trait AsyncExt: GraphNode {
    fn asyncify(self) -> Asyncify<Self>
    where
        Self: Sized,
    {
        Asyncify { src: self }
    }
}
