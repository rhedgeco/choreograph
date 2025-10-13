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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_correctly() {
        let mut marker = false;
        let task = Task::new(|| {
            marker = true;
            100
        });

        let output = task.execute();
        assert_eq!(output, 100);
        assert_eq!(marker, true);
    }

    #[test]
    fn wrap_returns_correct() {
        let task = Task::wrap(123);
        let out = task.execute();
        assert_eq!(out, 123);
    }
}
