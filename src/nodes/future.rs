use std::future::{ready, Future, Ready};

use futures::{executor::block_on, future::Shared, FutureExt as _};

use crate::Node;

pub struct ToFuture<Src> {
    src: Src,
}

impl<Src> Node for ToFuture<Src>
where
    Src: Node,
{
    type Output = Ready<Src::Output>;

    fn exec(self) -> Self::Output {
        ready(self.src.exec())
    }
}

pub struct ToShared<Src> {
    src: Src,
}

impl<Src> Node for ToShared<Src>
where
    Src: Node,
    Src::Output: Future,
    <Src::Output as Future>::Output: Clone,
{
    type Output = Shared<Src::Output>;

    fn exec(self) -> Self::Output {
        self.src.exec().shared()
    }
}

pub struct BlockOn<Src> {
    src: Src,
}

impl<Src> Node for BlockOn<Src>
where
    Src: Node,
    Src::Output: Future,
{
    type Output = <Src::Output as Future>::Output;

    fn exec(self) -> Self::Output {
        block_on(self.src.exec())
    }
}

impl<T: Node> FutureExt for T {}
pub trait FutureExt: Node {
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
