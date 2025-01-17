use derive_where::derive_where;

use crate::{Graph, GraphCtx};

/// An extension trait that allows for building a graph branch from any [`GraphNode`]
pub trait BranchExt: Graph {
    /// Returns a new [`GraphBranch`] whos `task` converts this graphs output into a new output
    fn branch<Output>(self, task: fn(Self::Output) -> Output) -> Branch<Output, Self>
    where
        Output: 'static,
    {
        Branch { task, source: self }
    }
}
impl<T: Graph> BranchExt for T {}

/// A graph that executes a task as a branch off of another graph
#[derive_where(Debug, Clone, Copy; Source)]
pub struct Branch<Output, Source>
where
    Source: Graph,
{
    task: fn(Source::Output) -> Output,
    source: Source,
}

impl<Output, Source> Graph for Branch<Output, Source>
where
    Source: Graph,
    Output: 'static,
{
    type Input = Source::Input;
    type Output = Output;

    fn execute_with_ctx(&self, ctx: &mut GraphCtx, input: Self::Input) -> Self::Output {
        let input = self.source.execute_with_ctx(ctx, input);
        (self.task)(input)
    }
}
