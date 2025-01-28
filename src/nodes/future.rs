use std::future::{ready, Future, Ready};

use futures::{future, FutureExt as _};

use crate::Node;

pub struct ToFuture<Src> {
    src: Src,
}

impl<Src> ToFuture<Src>
where
    Src: Node,
{
    pub fn new(src: Src) -> Self {
        Self { src }
    }
}

impl<Src> Node for ToFuture<Src>
where
    Src: Node,
{
    type Output = Ready<Src::Output>;

    fn call(self) -> Self::Output {
        ready(self.src.call())
    }
}

pub struct Shared<Src> {
    src: Src,
}

impl<Src> Shared<Src>
where
    Self: Node,
{
    pub fn new(src: Src) -> Self {
        Self { src }
    }
}

impl<Src> Node for Shared<Src>
where
    Src: Node,
    Src::Output: Future,
    <Src::Output as Future>::Output: Clone,
{
    type Output = future::Shared<Src::Output>;

    fn call(self) -> Self::Output {
        self.src.call().shared()
    }
}

impl<T: Node> FutureExt for T {}
pub trait FutureExt: Node {
    fn to_future(self) -> ToFuture<Self>
    where
        Self: Sized,
    {
        ToFuture::new(self)
    }

    fn shared(self) -> Shared<Self>
    where
        Self: Sized,
        Self::Output: Future,
        <Self::Output as Future>::Output: Clone,
    {
        Shared::new(self)
    }
}
