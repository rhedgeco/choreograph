use crate::TaskCache;

/// A trait that defines the structure of a part of a graph
///
/// GraphNodes have an Input type, Output type, and an function that executes the node
pub trait GraphNode: Copy + 'static {
    type Input: Clone + 'static;
    type Output: Clone + 'static;
    fn execute_cached(&self, cache: &mut TaskCache, input: Self::Input) -> Self::Output;
}

/// An extension trait that wraps [`GraphNode`]s and creates an easy execute function
pub trait NodeExt: GraphNode {
    /// Builds a context and executes a graph all the way through
    fn execute(&self, input: Self::Input) -> Self::Output {
        let mut cache = TaskCache::new();
        self.execute_cached(&mut cache, input)
    }
}
impl<T: GraphNode> NodeExt for T {}
