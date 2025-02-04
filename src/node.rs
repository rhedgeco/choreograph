pub trait Node {
    type Output;
    fn execute(self) -> Self::Output;
}
