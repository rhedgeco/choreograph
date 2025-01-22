use std::marker::PhantomData;

use crate::GraphNode;

pub struct Join<Out, Src1, Src2, F> {
    _types: PhantomData<fn() -> Out>,
    src1: Src1,
    src2: Src2,
    action: F,
}

impl<Out, Src1, Src2, F> GraphNode for Join<Out, Src1, Src2, F>
where
    Src1: GraphNode,
    Src2: GraphNode,
    F: FnOnce(Src1::Output, Src2::Output) -> Out,
{
    type Output = Out;

    fn execute(self) -> Self::Output {
        (self.action)(self.src1.execute(), self.src2.execute())
    }
}

impl<T: GraphNode> JoinExt for T {}
pub trait JoinExt: GraphNode {
    fn join<Src2>(
        self,
        src2: Src2,
    ) -> Join<
        (Self::Output, Src2::Output),
        Self,
        Src2,
        fn(Self::Output, Src2::Output) -> (Self::Output, Src2::Output),
    >
    where
        Self: Sized,
        Src2: GraphNode,
    {
        self.join_map(src2, |src1, src2| (src1, src2))
    }

    fn join_map<Out, Src2, F>(self, src2: Src2, action: F) -> Join<Out, Self, Src2, F>
    where
        Self: Sized,
        Src2: GraphNode,
        F: FnOnce(Self::Output, Src2::Output) -> Out,
    {
        Join {
            _types: PhantomData,
            src1: self,
            src2,
            action,
        }
    }
}
