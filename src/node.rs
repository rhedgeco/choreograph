use std::future::Future;

use futures::join;

pub struct Source<T> {
    value: T,
}

impl<T> Source<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> GraphNode<T> for Source<T> {
    fn execute(self) -> impl Future<Output = T> {
        async move { self.value }
    }
}

pub trait GraphNode<Out> {
    fn execute(self) -> impl Future<Output = Out>;
}

pub fn test_node() -> TestNodeBuilder<(), (), ()> {
    TestNodeBuilder {
        g1: (),
        g2: (),
        g3: (),
    }
}

pub struct TestNode<G1, G2, G3> {
    g1: G1,
    g2: G2,
    g3: G3,
}

impl<G1, G2, G3> GraphNode<u32> for TestNode<G1, G2, G3>
where
    G1: GraphNode<u32>,
    G2: GraphNode<u32>,
    G3: GraphNode<u32>,
{
    fn execute(self) -> impl Future<Output = u32> {
        async fn test_node(v1: u32, v2: u32, v3: u32) -> u32 {
            v1 + v2 + v3
        }

        async move {
            let (v1, v2, v3) = join!(self.g1.execute(), self.g2.execute(), self.g3.execute());
            test_node(v1, v2, v3).await
        }
    }
}

pub struct TestNodeBuilder<I1, I2, I3> {
    g1: I1,
    g2: I2,
    g3: I3,
}

impl<I2, I3> TestNodeBuilder<(), I2, I3> {
    pub fn link_value1<G>(self, node: G) -> TestNodeBuilder<G, I2, I3>
    where
        G: GraphNode<u32>,
    {
        TestNodeBuilder {
            g1: node,
            g2: self.g2,
            g3: self.g3,
        }
    }
}

impl<I1, I3> TestNodeBuilder<I1, (), I3> {
    pub fn link_value2<G>(self, node: G) -> TestNodeBuilder<I1, G, I3>
    where
        G: GraphNode<u32>,
    {
        TestNodeBuilder {
            g1: self.g1,
            g2: node,
            g3: self.g3,
        }
    }
}

impl<I1, I2> TestNodeBuilder<I1, I2, ()> {
    pub fn link_value3<G>(self, node: G) -> TestNodeBuilder<I1, I2, G>
    where
        G: GraphNode<u32>,
    {
        TestNodeBuilder {
            g1: self.g1,
            g2: self.g2,
            g3: node,
        }
    }
}

impl<G1, G2, G3> TestNodeBuilder<G1, G2, G3>
where
    G1: GraphNode<u32>,
    G2: GraphNode<u32>,
    G3: GraphNode<u32>,
{
    pub fn build(self) -> TestNode<G1, G2, G3> {
        TestNode {
            g1: self.g1,
            g2: self.g2,
            g3: self.g3,
        }
    }
}
