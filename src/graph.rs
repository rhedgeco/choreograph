pub trait GraphNode {
    type Output;
    fn execute(self) -> Self::Output;
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

impl<F, Out> GraphNode for Node<F>
where
    F: FnOnce() -> Out,
{
    type Output = Out;

    fn execute(self) -> Self::Output {
        (self.action)()
    }
}
