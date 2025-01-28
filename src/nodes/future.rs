use std::future::{ready, Future, Ready};

use futures::{future, FutureExt as _};

use crate::GraphNode;

pub struct ToFuture<Src> {
    src: Src,
}

impl<Src> GraphNode for ToFuture<Src>
where
    Src: GraphNode,
{
    type Output = Ready<Src::Output>;

    fn call(self) -> Self::Output {
        ready(self.src.call())
    }
}

pub struct Shared<Src> {
    src: Src,
}

impl<Src> GraphNode for Shared<Src>
where
    Src: GraphNode,
    Src::Output: Future,
    <Src::Output as Future>::Output: Clone,
{
    type Output = future::Shared<Src::Output>;

    fn call(self) -> Self::Output {
        self.src.call().shared()
    }
}

impl<T: GraphNode> FutureExt for T {}
pub trait FutureExt: GraphNode {
    fn to_future(self) -> ToFuture<Self>
    where
        Self: Sized,
    {
        ToFuture { src: self }
    }

    fn shared(self) -> Shared<Self>
    where
        Self: Sized,
        Self::Output: Future,
        <Self::Output as Future>::Output: Clone,
    {
        Shared { src: self }
    }
}
