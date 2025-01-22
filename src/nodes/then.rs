use std::marker::PhantomData;

use crate::GraphNode;

pub struct Then<Out, Src, F> {
    _types: PhantomData<fn() -> Out>,
    action: F,
    src: Src,
}

impl<Out, Src, F> GraphNode for Then<Out, Src, F>
where
    Src: GraphNode,
    F: FnOnce(Src::Output) -> Out,
{
    type Output = Out;

    fn execute(self) -> Self::Output {
        (self.action)(self.src.execute())
    }
}

impl<T: GraphNode> ThenExt for T {}
pub trait ThenExt: GraphNode {
    fn then<Out, F>(self, action: F) -> Then<Out, Self, F>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> Out,
    {
        Then {
            _types: PhantomData,
            action,
            src: self,
        }
    }
}
