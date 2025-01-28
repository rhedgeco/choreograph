use crate::GraphNode;

pub struct Action<F> {
    action: F,
}

impl<F> Action<F> {
    pub fn new<Out>(action: F) -> Self
    where
        F: FnOnce() -> Out,
    {
        Self { action }
    }
}

impl<F, Out> GraphNode for Action<F>
where
    F: FnOnce() -> Out,
{
    type Output = Out;

    fn call(self) -> Self::Output {
        (self.action)()
    }
}
