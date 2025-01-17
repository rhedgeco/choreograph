use std::{
    any::Any,
    fmt::Debug,
    hash::Hash,
    sync::atomic::{AtomicU64, Ordering},
};

use indexmap::IndexMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(u64);

impl TaskId {
    pub fn new() -> Self {
        static GEN: AtomicU64 = AtomicU64::new(0);
        Self(GEN.fetch_add(1, Ordering::Relaxed))
    }
}

impl AsRef<TaskId> for TaskId {
    fn as_ref(&self) -> &TaskId {
        self
    }
}

pub struct Task<Input, Output> {
    id: TaskId,
    task: fn(Input) -> Output,
}

impl<Input, Output> Task<Input, Output> {
    pub fn new(task: fn(Input) -> Output) -> Self {
        Self {
            id: TaskId::new(),
            task,
        }
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn execute(&self, input: Input) -> Output {
        (self.task)(input)
    }
}

impl<Input, Output> Eq for Task<Input, Output> {}
impl<Input, Output> PartialEq for Task<Input, Output> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Input, Output> Ord for Task<Input, Output> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl<Input, Output> PartialOrd for Task<Input, Output> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<Input, Output> Hash for Task<Input, Output> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<Input, Output> Copy for Task<Input, Output> {}
impl<Input, Output> Clone for Task<Input, Output> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            task: self.task,
        }
    }
}

impl<Input, Output> Debug for Task<Input, Output> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task")
            .field("id", &self.id)
            .field("task", &self.task)
            .finish()
    }
}

/// A cache for storing the ouput of a [`Task`].
#[derive(Debug, Default)]
pub struct TaskCache {
    cache: IndexMap<TaskId, Box<dyn Any>>,
}

impl TaskCache {
    /// Returns a new empty task cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the cached output from a task.
    ///
    /// The task will be executed only if it has not already been cached.
    /// In subsequent executions, the cached value from this run will be returned instead.
    pub fn execute_cached<Input: Clone, Output: 'static>(
        &mut self,
        input: Input,
        task: Task<Input, Output>,
    ) -> &Output {
        use indexmap::map::Entry as E;
        match self.cache.entry(task.id()) {
            E::Occupied(entry) => entry.into_mut(),
            E::Vacant(entry) => entry.insert(Box::new(task.execute(input))),
        }
        .downcast_ref::<Output>()
        .expect("invalid downcast to output type")
    }
}
