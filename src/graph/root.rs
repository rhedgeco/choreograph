use std::fmt::Debug;

use crate::{Task, TaskCache};

use super::GraphNode;

/// A graph that contains only one task
pub struct GraphRoot<Input, Output> {
    task: Task<Input, Output>,
}

impl<Input, Output> GraphRoot<Input, Output> {
    /// Returns a new graph root node with `task`
    pub fn new(task: fn(Input) -> Output) -> Self {
        Self {
            task: Task::new(task),
        }
    }
}

impl<Input, Output> GraphNode for GraphRoot<Input, Output>
where
    Input: Clone + 'static,
    Output: Clone + 'static,
{
    type Input = Input;
    type Output = Output;

    fn execute_cached(&self, cache: &mut TaskCache, input: Self::Input) -> Self::Output {
        cache.execute_cached(input, self.task).clone()
    }
}

impl<Input, Output> Copy for GraphRoot<Input, Output> {}
impl<Input, Output> Clone for GraphRoot<Input, Output> {
    fn clone(&self) -> Self {
        Self { task: self.task }
    }
}

impl<Input, Output> Debug for GraphRoot<Input, Output> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphRoot")
            .field("task", &self.task)
            .finish()
    }
}
