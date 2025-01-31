use crate::GraphNode;

pub struct Source<T>(T);

impl<T> Source<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> GraphNode for Source<T> {
    type Output = T;
    fn execute(self) -> Self::Output {
        self.0
    }
}
