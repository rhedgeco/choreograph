pub trait GraphNode {
    type Output;
    fn execute(self) -> Self::Output;
}
