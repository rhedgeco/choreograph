use std::any::Any;

use indexmap::IndexMap;

use crate::{task::TaskId, Task};

#[derive(Debug, Default)]
pub struct ExecContext {
    values: IndexMap<TaskId, Box<dyn Any>>,
}

impl ExecContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_store<Input: Clone, Output: 'static>(
        &mut self,
        input: Input,
        task: Task<Input, Output>,
    ) -> &Box<dyn Any> {
        use indexmap::map::Entry as E;
        match self.values.entry(task.id()) {
            E::Occupied(entry) => entry.into_mut(),
            E::Vacant(entry) => entry.insert(Box::new(task.execute(input.clone()))),
        }
    }
}

pub trait GraphKind: Copy {
    type Input: Clone + 'static;
    type Output: Clone + 'static;
    fn execute(&self, ctx: &mut ExecContext, input: Self::Input) -> Self::Output;
}

pub struct Root<Input, Output> {
    task: Task<Input, Output>,
}

impl<Input, Output> GraphKind for Root<Input, Output>
where
    Input: Clone + 'static,
    Output: Clone + 'static,
{
    type Input = Input;
    type Output = Output;

    fn execute(&self, ctx: &mut ExecContext, input: Self::Input) -> Self::Output {
        ctx.get_or_store(input, self.task)
            .downcast_ref::<Output>()
            .expect("valid type")
            .clone()
    }
}

impl<Input, Output> Copy for Root<Input, Output> {}
impl<Input, Output> Clone for Root<Input, Output> {
    fn clone(&self) -> Self {
        Self { task: self.task }
    }
}

pub struct Branch<Output, Source>
where
    Source: GraphKind,
{
    task: Task<Source::Output, Output>,
    source: Source,
}

impl<Output, Source> GraphKind for Branch<Output, Source>
where
    Output: Clone + 'static,
    Source: GraphKind,
{
    type Input = Source::Input;
    type Output = Output;

    fn execute(&self, ctx: &mut ExecContext, input: Self::Input) -> Self::Output {
        let input = self.source.execute(ctx, input);
        ctx.get_or_store(input, self.task)
            .downcast_ref::<Output>()
            .expect("valid type")
            .clone()
    }
}

impl<Output, Source> Copy for Branch<Output, Source> where Source: GraphKind {}
impl<Output, Source> Clone for Branch<Output, Source>
where
    Source: GraphKind,
{
    fn clone(&self) -> Self {
        Self {
            task: self.task,
            source: self.source,
        }
    }
}

pub struct Join<Output, Src1, Src2>
where
    Src1: GraphKind,
    Src2: GraphKind<Input = Src1::Input>,
{
    task: Task<(Src1::Output, Src2::Output), Output>,
    source1: Src1,
    source2: Src2,
}

impl<Output, Src1, Src2> GraphKind for Join<Output, Src1, Src2>
where
    Output: Clone + 'static,
    Src1: GraphKind,
    Src2: GraphKind<Input = Src1::Input>,
{
    type Input = Src1::Input;
    type Output = Output;

    fn execute(&self, ctx: &mut ExecContext, input: Self::Input) -> Self::Output {
        let input1 = self.source1.execute(ctx, input.clone());
        let input2 = self.source2.execute(ctx, input);
        ctx.get_or_store((input1, input2), self.task)
            .downcast_ref::<Output>()
            .expect("valid type")
            .clone()
    }
}

impl<Output, Src1, Src2> Copy for Join<Output, Src1, Src2>
where
    Src1: GraphKind,
    Src2: GraphKind<Input = Src1::Input>,
{
}
impl<Output, Src1, Src2> Clone for Join<Output, Src1, Src2>
where
    Src1: GraphKind,
    Src2: GraphKind<Input = Src1::Input>,
{
    fn clone(&self) -> Self {
        Self {
            task: self.task,
            source1: self.source1,
            source2: self.source2,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Graph<Kind> {
    kind: Kind,
}

impl<Kind: GraphKind> Graph<Kind> {
    pub fn execute(&self, input: Kind::Input) -> Kind::Output {
        let mut ctx = ExecContext::new();
        self.kind.execute(&mut ctx, input)
    }
}

impl<Input, Output> Graph<Root<Input, Output>>
where
    Input: Clone + 'static,
    Output: Clone + 'static,
{
    pub fn root(task: fn(Input) -> Output) -> Self {
        Self {
            kind: Root {
                task: Task::new(task),
            },
        }
    }

    pub fn branch<NewOut>(
        self,
        task: fn(Output) -> NewOut,
    ) -> Graph<Branch<NewOut, Root<Input, Output>>> {
        Graph {
            kind: Branch {
                task: Task::new(task),
                source: self.kind,
            },
        }
    }

    pub fn join<NewOut, Other>(
        self,
        other: Graph<Other>,
        task: fn((Output, Other::Output)) -> NewOut,
    ) -> Graph<Join<NewOut, Root<Input, Output>, Other>>
    where
        Other: GraphKind<Input = Input>,
    {
        Graph {
            kind: Join {
                task: Task::new(task),
                source1: self.kind,
                source2: other.kind,
            },
        }
    }
}

impl<Output, Source> Graph<Branch<Output, Source>>
where
    Output: Clone + 'static,
    Source: GraphKind,
{
    pub fn branch<NewOut>(
        self,
        task: fn(Output) -> NewOut,
    ) -> Graph<Branch<NewOut, Branch<Output, Source>>> {
        Graph {
            kind: Branch {
                task: Task::new(task),
                source: self.kind,
            },
        }
    }

    pub fn join<NewOut, Other>(
        self,
        other: Graph<Other>,
        task: fn((Output, Other::Output)) -> NewOut,
    ) -> Graph<Join<NewOut, Branch<Output, Source>, Other>>
    where
        Other: GraphKind<Input = Source::Input>,
    {
        Graph {
            kind: Join {
                task: Task::new(task),
                source1: self.kind,
                source2: other.kind,
            },
        }
    }
}

impl<Output, Src1, Src2> Graph<Join<Output, Src1, Src2>>
where
    Output: Clone + 'static,
    Src1: GraphKind,
    Src2: GraphKind<Input = Src1::Input>,
{
    pub fn branch<NewOut>(
        self,
        task: fn(Output) -> NewOut,
    ) -> Graph<Branch<NewOut, Join<Output, Src1, Src2>>> {
        Graph {
            kind: Branch {
                task: Task::new(task),
                source: self.kind,
            },
        }
    }

    pub fn join<NewOut, Other>(
        self,
        other: Graph<Other>,
        task: fn((Output, Other::Output)) -> NewOut,
    ) -> Graph<Join<NewOut, Join<Output, Src1, Src2>, Other>>
    where
        Other: GraphKind<Input = Src1::Input>,
    {
        Graph {
            kind: Join {
                task: Task::new(task),
                source1: self.kind,
                source2: other.kind,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_branch_join() {
        const TEST_INPUT: u32 = 50;
        const TEST_OUTPUT: u32 = ((100 + TEST_INPUT + 20) + (100 + TEST_INPUT - 10)) / 2;

        let root_task = Graph::root(|v: u32| v + 100);
        let add_task = root_task.branch(|v| v + 20);
        let sub_task = root_task.branch(|v| v - 10);
        let join_task = add_task.join(sub_task, |(v1, v2)| (v1 + v2) / 2);

        // execute the task and check if it is valid
        let output = join_task.execute(TEST_INPUT);
        assert_eq!(output, TEST_OUTPUT);
    }
}
