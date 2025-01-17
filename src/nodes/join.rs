use derive_where::derive_where;

use crate::Task;

use crate::{Graph, GraphCtx};

pub trait JoinExt: Graph {
    /// Returns a new [`Join`] whos `task` takes the outputs of
    /// `self` and `other` and combines them into a single output.
    ///
    /// `self` and `other` must have the same input for them to be joined
    fn join<Output, Other>(
        self,
        other: Other,
        task: fn((Self::Output, Other::Output)) -> Output,
    ) -> Join<Output, Self, Other>
    where
        Self::Input: Clone,
        Other: Graph<Input = Self::Input>,
    {
        Join {
            task: Task::new(task),
            src1: self,
            src2: other,
        }
    }
}
impl<T: Graph> JoinExt for T {}

/// A graph that executes a task that joins the outputs of two other graphs
#[derive_where(Debug, Clone, Copy; Src1, Src2)]
pub struct Join<Output, Src1, Src2>
where
    Src1: Graph,
    Src2: Graph<Input = Src1::Input>,
{
    task: Task<(Src1::Output, Src2::Output), Output>,
    src1: Src1,
    src2: Src2,
}

impl<Output, Src1, Src2> Graph for Join<Output, Src1, Src2>
where
    Src1: Graph,
    Src1::Input: Clone,
    Src2: Graph<Input = Src1::Input>,
{
    type Input = Src1::Input;
    type Output = Output;

    fn execute_with_ctx(&self, ctx: &mut GraphCtx, input: Self::Input) -> Self::Output {
        let input1 = self.src1.execute_with_ctx(ctx, input.clone());
        let input2 = self.src2.execute_with_ctx(ctx, input);
        self.task.execute((input1, input2))
    }
}
