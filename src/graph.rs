use std::{
    any::Any,
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

use indexmap::{indexmap, IndexMap};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(u64);

impl TaskId {
    pub fn new() -> Self {
        static GEN: AtomicU64 = AtomicU64::new(0);
        Self(GEN.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone)]
struct Stitcher {
    initial_id: TaskId,
    initial_stitch: fn(&dyn Any) -> Box<dyn Any>,
    branches: IndexMap<TaskId, fn(&dyn Any, Box<dyn Any>) -> Box<dyn Any>>,
}

impl Stitcher {
    pub fn new<I, O>(graph: &Graph<I, O>) -> Self
    where
        I: Clone + 'static,
        O: Clone + 'static,
    {
        Self {
            initial_id: graph.output_id(),
            initial_stitch: |output| {
                let output = output.downcast_ref::<O>().unwrap().clone();
                Box::new(output)
            },
            branches: IndexMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct TaskEntry {
    id: TaskId,
    task: *const (),
    runner: fn(*const (), Box<dyn Any>) -> Box<dyn Any>,
    stitcher: Option<Stitcher>,
}

impl TaskEntry {
    pub fn run(&self, input: Box<dyn Any>) -> Box<dyn Any> {
        (self.runner)(self.task, input)
    }
}

impl TaskEntry {
    pub fn new<I, O>(task: fn(I) -> O, stitcher: Option<Stitcher>) -> Self
    where
        I: Clone + 'static,
        O: Clone + 'static,
    {
        Self {
            id: TaskId::new(),
            task: task as *const (),
            runner: |task, input| {
                let input = input.downcast::<I>().unwrap();
                let task: fn(I) -> O = unsafe { core::mem::transmute(task) };
                Box::new(task(*input))
            },
            stitcher,
        }
    }
}

pub struct Graph<Input, Output> {
    _phantom: PhantomData<fn(Input) -> Output>,
    tasks: IndexMap<TaskId, TaskEntry>,
}

impl<Input, Output> Graph<Input, Output>
where
    Input: Clone + 'static,
    Output: Clone + 'static,
{
    fn output_id(&self) -> TaskId {
        *self.tasks.last().unwrap().0
    }

    pub fn task() -> TaskBuilder<Input, Output> {
        TaskBuilder {
            _phantom: PhantomData,
        }
    }

    pub fn execute(&self, input: Input) -> Output {
        let mut store = IndexMap::<TaskId, Box<dyn Any>>::new();
        for entry in self.tasks.values() {
            let task_input: Box<dyn Any> = match &entry.stitcher {
                None => Box::new(input.clone()),
                Some(stitcher) => {
                    let initial = store.get(&stitcher.initial_id).unwrap().as_ref();
                    let mut stitched = (stitcher.initial_stitch)(initial);
                    for (id, stitch) in stitcher.branches.iter() {
                        let branch_input = store.get(id).unwrap().as_ref();
                        stitched = stitch(branch_input, stitched);
                    }
                    stitched
                }
            };

            let output = entry.run(task_input);
            store.insert(entry.id, output);
        }

        let (_, output) = store.pop().unwrap();
        *output.downcast().unwrap()
    }
}

pub struct TaskBuilder<Input, Output> {
    _phantom: PhantomData<fn(Input) -> Output>,
}

impl<Input, Output> TaskBuilder<Input, Output>
where
    Input: Clone + 'static,
    Output: Clone + 'static,
{
    pub fn build(self, task: fn(Input) -> Output) -> Graph<Input, Output> {
        let entry = TaskEntry::new(task, None);
        Graph {
            _phantom: PhantomData,
            tasks: indexmap! {entry.id => entry},
        }
    }

    pub fn branch(self, graph: &Graph<Input, Output>) -> BranchTaskBuilder<Input, Output> {
        BranchTaskBuilder {
            _phantom: PhantomData,
            tasks: graph.tasks.clone(),
            stitcher: Stitcher::new(graph),
        }
    }
}

pub struct BranchTaskBuilder<Input, Output> {
    _phantom: PhantomData<fn(Input) -> Output>,
    tasks: IndexMap<TaskId, TaskEntry>,
    stitcher: Stitcher,
}

impl<Input, Output> BranchTaskBuilder<Input, Output>
where
    Input: Clone + 'static,
    Output: Clone + 'static,
{
    pub fn branch<Out2>(
        mut self,
        graph: &Graph<Input, Out2>,
    ) -> BranchTaskBuilder<Input, (Output, Out2)>
    where
        Out2: Clone + 'static,
    {
        // add branch stitcher for incoming types
        self.stitcher
            .branches
            .insert(graph.output_id(), |new, old| {
                let new = new.downcast_ref::<Out2>().unwrap().clone();
                let old = *old.downcast::<Output>().unwrap();
                Box::new((old, new))
            });

        // return new branch builder
        BranchTaskBuilder {
            _phantom: PhantomData,
            tasks: [self.tasks, graph.tasks.clone()]
                .into_iter()
                .kmerge_by(|(id1, _), (id2, _)| id1 <= id2)
                .dedup_by(|(id1, _), (id2, _)| id1 == id2)
                .collect::<IndexMap<_, _>>(),
            stitcher: self.stitcher,
        }
    }

    pub fn build<Out2>(mut self, task: fn(Output) -> Out2) -> Graph<Input, Out2>
    where
        Out2: Clone + 'static,
    {
        let entry = TaskEntry::new(task, Some(self.stitcher));
        self.tasks.insert(entry.id, entry);
        Graph {
            _phantom: PhantomData,
            tasks: self.tasks,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_branch_join() {
        // create target input and output values
        const TEST_INPUT: u32 = 50;
        const TEST_OUTPUT: u32 = ((100 + TEST_INPUT + 20) + (100 + TEST_INPUT - 10)) / 2;

        // create the graphs root task
        let root = Graph::task().build(|value: u32| 100 + value);

        // create a branching add task off of root
        let add_task = Graph::task().branch(&root).build(|value| value + 20);

        // create a branching subtract task also off of root
        let sub_task = Graph::task().branch(&root).build(|value| value - 10);

        // join the result of the add and subtract task by branching off of both
        let join_task = Graph::task()
            .branch(&add_task)
            .branch(&sub_task)
            .build(|(v1, v2)| (v1 + v2) / 2);

        // execute the task and check if it is valid
        let output = join_task.execute(TEST_INPUT);
        assert_eq!(output, TEST_OUTPUT);
    }
}
