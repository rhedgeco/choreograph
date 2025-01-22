use crate::GraphNode;

pub struct Source<T> {
    value: T,
}

impl<T> Source<T> {
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> GraphNode for Source<T> {
    type Output = T;
    fn execute(self) -> Self::Output {
        self.value
    }
}
