use std::future::{ready, Future, Ready};

use futures::{executor::block_on, future::Shared, FutureExt as _};

use crate::NodeExec;

pub struct ToFuture<Src> {
    src: Src,
}

impl<Src> NodeExec for ToFuture<Src>
where
    Src: NodeExec,
{
    type Output = Ready<Src::Output>;

    fn exec(self) -> Self::Output {
        ready(self.src.exec())
    }
}

pub struct ToShared<Src> {
    src: Src,
}

impl<Src> NodeExec for ToShared<Src>
where
    Src: NodeExec,
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

impl<Src> NodeExec for BlockOn<Src>
where
    Src: NodeExec,
    Src::Output: Future,
{
    type Output = <Src::Output as Future>::Output;

    fn exec(self) -> Self::Output {
        block_on(self.src.exec())
    }
}

impl<T: NodeExec> FutureExt for T {}
pub trait FutureExt: NodeExec {
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
