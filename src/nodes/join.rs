use crate::Node;

pub struct Join<Src1, Src2> {
    src1: Src1,
    src2: Src2,
}

impl<Src1, Src2> Node for Join<Src1, Src2>
where
    Src1: Node,
    Src2: Node,
{
    type Output = (Src1::Output, Src2::Output);

    fn call(self) -> Self::Output {
        (self.src1.call(), self.src2.call())
    }
}

impl<T: Node> JoinExt for T {}
pub trait JoinExt: Node {
    fn join<Src2>(self, src2: Src2) -> Join<Self, Src2>
    where
        Self: Sized,
        Src2: Node,
    {
        Join { src1: self, src2 }
    }
}
