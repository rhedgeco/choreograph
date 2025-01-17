use std::any::Any;

use indexmap::IndexMap;

use crate::{task::TaskId, Task};

/// A context manager that stores the results of different tasks execution data
#[derive(Debug, Default)]
pub struct OutputCache {
    values: IndexMap<TaskId, Box<dyn Any>>,
}

impl OutputCache {
    /// Returns a new empty execution context
    pub fn new() -> Self {
        Self::default()
    }

    /// Tries to execute a task and store its output
    pub fn get_or_store_task<Input: Clone, Output: 'static>(
        &mut self,
        input: Input,
        task: Task<Input, Output>,
    ) -> &Output {
        use indexmap::map::Entry as E;
        match self.values.entry(task.id()) {
            E::Occupied(entry) => entry.into_mut(),
            E::Vacant(entry) => entry.insert(Box::new(task.execute(input.clone()))),
        }
        .downcast_ref::<Output>()
        .expect("Invalid conversion of output back to original type")
    }
}

/// A trait that defines the structure of a part of a graph
///
/// GraphNodes have an Input type, Output type, and an function that executes the node
pub trait GraphNode: Copy + 'static {
    type Input: Clone + 'static;
    type Output: Clone + 'static;
    fn execute_with_cache(&self, cache: &mut OutputCache, input: Self::Input) -> Self::Output;
}

/// An extension trait that wraps [`GraphNode`]s and creates an easy execute function
pub trait NodeExt: GraphNode {
    /// Builds a context and executes a graph all the way through
    fn execute(&self, input: Self::Input) -> Self::Output {
        let mut cache = OutputCache::new();
        self.execute_with_cache(&mut cache, input)
    }
}
impl<T: GraphNode> NodeExt for T {}
