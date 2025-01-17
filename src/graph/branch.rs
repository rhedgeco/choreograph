use std::fmt::Debug;

use crate::{Task, TaskCache};

use super::GraphNode;

/// An extension trait that allows for building a graph branch from any [`GraphNode`]
pub trait BranchExt: GraphNode {
    /// Returns a new [`GraphBranch`] whos `task` converts this graphs output into a new output
    fn branch<Output>(self, task: fn(Self::Output) -> Output) -> GraphBranch<Output, Self> {
        GraphBranch {
            task: Task::new(task),
            source: self,
        }
    }
}
impl<T: GraphNode> BranchExt for T {}

/// A graph that executes a task as a branch off of another graph
pub struct GraphBranch<Output, Source>
where
    Source: GraphNode,
{
    task: Task<Source::Output, Output>,
    source: Source,
}

impl<Output, Source> GraphNode for GraphBranch<Output, Source>
where
    Output: Clone + 'static,
    Source: GraphNode,
{
    type Input = Source::Input;
    type Output = Output;

    fn execute_cached(&self, cache: &mut TaskCache, input: Self::Input) -> Self::Output {
        let input = self.source.execute_cached(cache, input);
        cache.execute_cached(input, self.task).clone()
    }
}

impl<Output, Source> Copy for GraphBranch<Output, Source> where Source: GraphNode {}
impl<Output, Source> Clone for GraphBranch<Output, Source>
where
    Source: GraphNode,
{
    fn clone(&self) -> Self {
        Self {
            task: self.task,
            source: self.source,
        }
    }
}

impl<Output, Source> Debug for GraphBranch<Output, Source>
where
    Source: GraphNode + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphBranch")
            .field("task", &self.task)
            .field("source", &self.source)
            .finish()
    }
}
