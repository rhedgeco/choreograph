use crate::GraphNode;

pub struct Data<T>(T);

impl<T> Data<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> GraphNode for Data<T> {
    type Output = T;
    fn execute(self) -> Self::Output {
        self.0
    }
}
