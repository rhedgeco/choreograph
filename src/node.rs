pub trait Node {
    type Output;
    fn exec(self) -> Self::Output;
}
