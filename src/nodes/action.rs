use crate::Node;

pub struct Action<F> {
    action: F,
}

impl<F, Out> Action<F>
where
    F: FnOnce() -> Out,
{
    pub fn new(action: F) -> Self {
        Self { action }
    }
}

impl<F, Out> Node for Action<F>
where
    F: FnOnce() -> Out,
{
    type Output = Out;

    fn exec(self) -> Self::Output {
        (self.action)()
    }
}
