use crate::Node;

pub struct Task<F> {
    task: F,
}

impl<F> Task<F> {
    pub fn new<Out>(task: F) -> Self
    where
        F: FnOnce() -> Out,
    {
        Self { task }
    }
}

impl<T> Task<T> {
    pub fn wrap(data: T) -> Task<impl FnOnce() -> T> {
        Task::new(|| data)
    }
}

impl<F, Out> Node for Task<F>
where
    F: FnOnce() -> Out,
{
    type Output = Out;

    fn execute(self) -> Self::Output {
        (self.task)()
    }
}
