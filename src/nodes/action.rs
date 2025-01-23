use std::marker::PhantomData;

use crate::GraphNode;

pub struct Action<Out, F> {
    _types: PhantomData<fn() -> Out>,
    action: F,
}

impl<Out, F> Action<Out, F> {
    pub fn new(action: F) -> Self
    where
        F: FnOnce() -> Out,
    {
        Self {
            _types: PhantomData,
            action,
        }
    }
}

impl<Out, F> GraphNode for Action<Out, F>
where
    F: FnOnce() -> Out,
{
    type Output = Out;

    fn execute(self) -> Self::Output {
        (self.action)()
    }
}
