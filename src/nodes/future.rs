use std::future::{ready, Future, Ready};

use futures::{executor::block_on, future::Shared, FutureExt as _};

use crate::GraphNode;

pub struct ToFuture<Src> {
    src: Src,
}

impl<Src> GraphNode for ToFuture<Src>
where
    Src: GraphNode,
{
    type Output = Ready<Src::Output>;

    fn execute(self) -> Self::Output {
        ready(self.src.execute())
    }
}

pub struct ToShared<Src> {
    src: Src,
}

impl<Src> GraphNode for ToShared<Src>
where
    Src: GraphNode,
    Src::Output: Future,
    <Src::Output as Future>::Output: Clone,
{
    type Output = Shared<Src::Output>;

    fn execute(self) -> Self::Output {
        self.src.execute().shared()
    }
}

pub struct BlockOn<Src> {
    src: Src,
}

impl<Src> GraphNode for BlockOn<Src>
where
    Src: GraphNode,
    Src::Output: Future,
{
    type Output = <Src::Output as Future>::Output;

    fn execute(self) -> Self::Output {
        block_on(self.src.execute())
    }
}

impl<T: GraphNode> FutureExt for T {}
pub trait FutureExt: GraphNode {
    fn future(self) -> ToFuture<Self>
    where
        Self: Sized,
    {
        ToFuture { src: self }
    }

    fn shared(self) -> ToShared<Self>
    where
        Self: Sized,
        Self::Output: Future,
        <Self::Output as Future>::Output: Clone,
    {
        ToShared { src: self }
    }

    fn block_on(self) -> BlockOn<Self>
    where
        Self: Sized,
        Self::Output: Future,
    {
        BlockOn { src: self }
    }
}
