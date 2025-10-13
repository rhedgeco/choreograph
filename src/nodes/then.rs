use crate::Node;

pub struct Then<Src, F> {
    action: F,
    src: Src,
}

impl<Src, F, Out> Node for Then<Src, F>
where
    Src: Node,
    F: FnOnce(Src::Output) -> Out,
{
    type Output = Out;

    fn execute(self) -> Self::Output {
        (self.action)(self.src.execute())
    }
}

impl<T: Node> ThenExt for T {}
pub trait ThenExt: Node {
    fn then<F, Out>(self, action: F) -> Then<Self, F>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> Out,
    {
        Then { action, src: self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::nodes::Task;

    use super::*;

    #[test]
    fn executes_in_order() {
        let marker = AtomicU32::new(5);
        let task = Task::wrap(5)
            .then(|value| {
                let old = marker.fetch_add(value, Ordering::Relaxed);
                assert_eq!(old, 5);
                value + 10
            })
            .then(|value| {
                let old = marker.fetch_add(value, Ordering::Relaxed);
                assert_eq!(old, 10);
                value + 5
            });

        let task_out = task.execute();
        assert_eq!(task_out, 20);

        let marker_out = marker.load(Ordering::Relaxed);
        assert_eq!(marker_out, 25);
    }
}
