use std::{cell::Cell, future::Future};

use crate::GraphNode;

pub struct Asyncify<Src> {
    src: Src,
}

impl<Src> GraphNode for Asyncify<Src>
where
    Src: GraphNode,
{
    type Output = Async<Src::Output>;

    fn execute(self) -> Self::Output {
        Async(Cell::new(Some(self.src.execute())))
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

pub struct Async<T>(Cell<Option<T>>);

impl<T> Future for Async<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.0.take() {
            Some(value) => std::task::Poll::Ready(value),
            None => panic!("poll called after future had completed"),
        }
    }
}
