use std::future::{ready, Future, Ready};

use futures::{executor::block_on, future::Shared, FutureExt as _};

use crate::Node;

pub struct Awaitable<Src> {
    src: Src,
}

impl<Src> Node for Awaitable<Src>
where
    Src: Node,
{
    type Output = Ready<Src::Output>;

    fn execute(self) -> Self::Output {
        ready(self.src.execute())
    }
}

pub struct Sharable<Src> {
    src: Src,
}

impl<Src> Node for Sharable<Src>
where
    Src: Node,
    Src::Output: Future,
    <Src::Output as Future>::Output: Clone,
{
    type Output = Shared<Src::Output>;

    fn execute(self) -> Self::Output {
        self.src.execute().shared()
    }
}

pub struct Blocking<Src> {
    src: Src,
}

impl<Src> Node for Blocking<Src>
where
    Src: Node,
    Src::Output: Future,
{
    type Output = <Src::Output as Future>::Output;

    fn execute(self) -> Self::Output {
        block_on(self.src.execute())
    }
}

impl<T: Node> FutureExt for T {}
pub trait FutureExt: Node {
    fn awaitable(self) -> Awaitable<Self>
    where
        Self: Sized,
    {
        Awaitable { src: self }
    }

    fn sharable(self) -> Sharable<Self>
    where
        Self: Sized,
        Self::Output: Future,
        <Self::Output as Future>::Output: Clone,
    {
        Sharable { src: self }
    }

    fn blocking(self) -> Blocking<Self>
    where
        Self: Sized,
        Self::Output: Future,
    {
        Blocking { src: self }
    }
}
