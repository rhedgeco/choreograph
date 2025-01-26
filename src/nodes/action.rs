use std::marker::PhantomData;

use crate::Node;

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

impl<Out, F> Node for Action<Out, F>
where
    F: FnOnce() -> Out,
{
    type Output = Out;

    fn call(self) -> Self::Output {
        (self.action)()
    }
}
