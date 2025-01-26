use crate::GraphNode;

pub struct Join<Src1, Src2> {
    src1: Src1,
    src2: Src2,
}

impl<Src1, Src2> Join<Src1, Src2>
where
    Src1: GraphNode,
    Src2: GraphNode,
{
    pub fn new(src1: Src1, src2: Src2) -> Self {
        Self { src1, src2 }
    }
}

impl<Src1, Src2> GraphNode for Join<Src1, Src2>
where
    Src1: GraphNode,
    Src2: GraphNode,
{
    type Output = (Src1::Output, Src2::Output);

    fn execute(self) -> Self::Output {
        (self.src1.execute(), self.src2.execute())
    }
}

impl<T: GraphNode> JoinExt for T {}
pub trait JoinExt: GraphNode {
    fn join<Src2>(self, src2: Src2) -> Join<Self, Src2>
    where
        Self: Sized,
        Src2: GraphNode,
    {
        Join::new(self, src2)
    }
}
