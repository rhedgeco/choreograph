pub trait GraphNode {
    type Output;
    fn execute(self) -> Self::Output;
}

pub trait IntoGraph {
    type Kind: GraphNode;
    fn into(value: Self) -> Self::Kind;
}
