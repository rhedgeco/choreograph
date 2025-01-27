use std::future::Future;

use futures::{future, FutureExt as _};

use crate::Node;

use super::ThenExt;

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
    fn awaitable(self) -> impl Node<Output = impl Future<Output = Self::Output>>
    where
        Self: Sized,
    {
        self.then(|value| async move { value })
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
