use derive_where::derive_where;

use crate::Task;

use super::{Graph, GraphCtx};

/// A node that that represents the entrypoint for a graph
#[derive_where(Debug, Clone, Copy)]
pub struct Entry<Input, Output> {
    task: Task<Input, Output>,
}

impl<Input, Output> Entry<Input, Output> {
    /// Returns a new graph entry node with `task`
    pub fn new(task: fn(Input) -> Output) -> Self {
        Self {
            task: Task::new(task),
        }
    }
}

impl<Input, Output> Graph for Entry<Input, Output> {
    type Input = Input;
    type Output = Output;

    fn execute_with_ctx(&self, _: &mut GraphCtx, input: Self::Input) -> Self::Output {
        self.task.execute(input)
    }
}
