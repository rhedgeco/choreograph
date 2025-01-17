use std::fmt::Debug;

use crate::{Task, TaskCache};

use super::GraphNode;

/// An extension trait that allows for building a graph branch from any [`GraphNode`]
pub trait AlwaysExt: GraphNode {
    /// Returns a new [`GraphAlways`] whos `task` converts this graphs output into a new output
    ///
    /// This is very similar to [`GraphBranch`](super::GraphBranch) except it will always
    /// re-execute its task every time it is called in a graph
    fn always<Output>(self, task: fn(Self::Output) -> Output) -> GraphAlways<Output, Self> {
        GraphAlways {
            task: Task::new(task),
            source: self,
        }
    }
}
impl<T: GraphNode> AlwaysExt for T {}

/// A graph node that always re-executes its task every time its called
pub struct GraphAlways<Output, Source>
where
    Source: GraphNode,
{
    task: Task<Source::Output, Output>,
    source: Source,
}

impl<Output, Source> GraphNode for GraphAlways<Output, Source>
where
    Output: Clone + 'static,
    Source: GraphNode,
{
    type Input = Source::Input;
    type Output = Output;

    fn execute_cached(&self, cache: &mut TaskCache, input: Self::Input) -> Self::Output {
        let input = self.source.execute_cached(cache, input);
        self.task.execute(input)
    }
}

impl<Output, Source> Copy for GraphAlways<Output, Source> where Source: GraphNode {}
impl<Output, Source> Clone for GraphAlways<Output, Source>
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

impl<Output, Source> Debug for GraphAlways<Output, Source>
where
    Source: GraphNode + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphAlways")
            .field("task", &self.task)
            .field("source", &self.source)
            .finish()
    }
}
