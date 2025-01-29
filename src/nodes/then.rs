use crate::GraphNode;

pub struct Then<Src, F> {
    action: F,
    src: Src,
}

impl<Src, F, Out> GraphNode for Then<Src, F>
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
    fn then<F, Out>(self, action: F) -> Then<Self, F>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> Out,
    {
        Then { action, src: self }
    }
}
