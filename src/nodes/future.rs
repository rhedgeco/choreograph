use std::future::Future;

use futures::FutureExt;

use crate::Graph;

pub trait SharedExt: Graph {
    /// Wraps a [`Graph`] that produces a future into a clonable handle
    fn shared(self) -> Shared<Self>
    where
        Self::Output: Future,
        <Self::Output as Future>::Output: Clone,
    {
        Shared { source: self }
    }
}
impl<T: Graph> SharedExt for T {}

#[derive(Debug, Clone, Copy)]
pub struct Shared<Source> {
    source: Source,
}

impl<Source: Graph> Graph for Shared<Source>
where
    Source::Output: Future,
    <Source::Output as Future>::Output: Clone,
{
    type Input = Source::Input;
    type Output = futures::future::Shared<Source::Output>;

    fn execute_with_ctx(&self, ctx: &mut crate::GraphCtx, input: Self::Input) -> Self::Output {
        self.source.execute_with_ctx(ctx, input).shared()
    }
}
