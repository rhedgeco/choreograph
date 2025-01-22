use std::future::Future;

use futures::{future, FutureExt};

use crate::GraphNode;

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

    fn execute(self) -> Self::Output {
        self.src.execute().shared()
    }
}

impl<T: GraphNode> SharedExt for T {}
pub trait SharedExt: GraphNode {
    fn shared(self) -> Shared<Self>
    where
        Self: Sized,
        Self::Output: Future,
        <Self::Output as Future>::Output: Clone,
    {
        Shared { src: self }
    }
}
