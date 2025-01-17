use std::fmt::Debug;

use crate::Task;

use super::{node::ExecContext, GraphNode};

/// An extension trait that allows for building a graph join between two [`GraphNode`]s
///
/// `self` and `other` must have the same input for them to be joined
pub trait JoinExt: GraphNode {
    /// Returns a new [`GraphJoin`] whos `task` takes the outputs of
    /// `self` and `other` and combines them into a single output.
    ///
    /// `self` and `other` must have the same input for them to be joined
    fn join<Output, Other>(
        self,
        other: Other,
        task: fn((Self::Output, Other::Output)) -> Output,
    ) -> GraphJoin<Output, Self, Other>
    where
        Output: Clone + 'static,
        Other: GraphNode<Input = Self::Input>,
    {
        GraphJoin {
            task: Task::new(task),
            src1: self,
            src2: other,
        }
    }
}
impl<T: GraphNode> JoinExt for T {}

/// A graph that executes a task that joins the outputs of two other graphs
pub struct GraphJoin<Output, Src1, Src2>
where
    Src1: GraphNode,
    Src2: GraphNode<Input = Src1::Input>,
{
    task: Task<(Src1::Output, Src2::Output), Output>,
    src1: Src1,
    src2: Src2,
}

impl<Output, Src1, Src2> GraphNode for GraphJoin<Output, Src1, Src2>
where
    Output: Clone + 'static,
    Src1: GraphNode,
    Src2: GraphNode<Input = Src1::Input>,
{
    type Input = Src1::Input;
    type Output = Output;

    fn execute_with_context(&self, ctx: &mut ExecContext, input: Self::Input) -> Self::Output {
        let input1 = self.src1.execute_with_context(ctx, input.clone());
        let input2 = self.src2.execute_with_context(ctx, input);
        ctx.get_or_store((input1, input2), self.task).clone()
    }
}

impl<Output, Src1, Src2> Copy for GraphJoin<Output, Src1, Src2>
where
    Src1: GraphNode,
    Src2: GraphNode<Input = Src1::Input>,
{
}
impl<Output, Src1, Src2> Clone for GraphJoin<Output, Src1, Src2>
where
    Src1: GraphNode,
    Src2: GraphNode<Input = Src1::Input>,
{
    fn clone(&self) -> Self {
        Self {
            task: self.task,
            src1: self.src1,
            src2: self.src2,
        }
    }
}

impl<Output, Src1, Src2> Debug for GraphJoin<Output, Src1, Src2>
where
    Src1: GraphNode + Debug,
    Src2: GraphNode<Input = Src1::Input> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphJoin")
            .field("task", &self.task)
            .field("src1", &self.src1)
            .field("src2", &self.src2)
            .finish()
    }
}
