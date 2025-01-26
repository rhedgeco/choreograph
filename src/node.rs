pub trait Node {
    type Output;
    fn call(self) -> Self::Output;
}
