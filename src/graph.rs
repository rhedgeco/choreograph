pub trait GraphNode {
    type Output;
    fn call(self) -> Self::Output;
}
