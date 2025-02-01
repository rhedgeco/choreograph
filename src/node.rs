pub trait NodeExec {
    type Output;
    fn exec(self) -> Self::Output;
}

pub struct Node<F> {
    action: F,
}

impl<F, Out> Node<F>
where
    F: FnOnce() -> Out,
{
    pub fn new(action: F) -> Self {
        Self { action }
    }
}

impl<F, Out> NodeExec for Node<F>
where
    F: FnOnce() -> Out,
{
    type Output = Out;

    fn exec(self) -> Self::Output {
        (self.action)()
    }
}
