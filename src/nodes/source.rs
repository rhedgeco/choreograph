use crate::Node;

pub struct Source<T>(T);

impl<T> Source<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Node for Source<T> {
    type Output = T;
    fn call(self) -> Self::Output {
        self.0
    }
}
