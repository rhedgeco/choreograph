use crate::GraphNode;

pub struct Source<F> {
    action: F,
}

impl<F> Source<F> {
    pub fn new<Out>(action: F) -> Self
    where
        F: FnOnce() -> Out,
    {
        Self { action }
    }
}

impl<F, Out> GraphNode for Source<F>
where
    F: FnOnce() -> Out,
{
    type Output = Out;

    fn execute(self) -> Self::Output {
        (self.action)()
    }
}
