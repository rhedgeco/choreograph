use std::sync::Arc;

use crate::{pool::ThreadPool, Node, ParNode};

pub struct Join<Src1, Src2> {
    src1: Src1,
    src2: Src2,
}

impl<Src1, Src2> Join<Src1, Src2> {
    pub fn new(src1: Src1, src2: Src2) -> Self
    where
        Self: Node,
    {
        Self { src1, src2 }
    }
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

pub struct ParJoin<Src1, Src2> {
    src1: Src1,
    src2: Src2,
}

impl<Src1, Src2> ParJoin<Src1, Src2> {
    pub fn new(src1: Src1, src2: Src2) -> Self
    where
        Self: ParNode,
    {
        Self { src1, src2 }
    }
}

impl<Src1, Src2> ParNode for ParJoin<Src1, Src2>
where
    Src1: ParNode,
    Src2: ParNode,
{
    type Output = (Src1::Output, Src2::Output);

    fn call_par(self, pool: Arc<impl ThreadPool>) -> Self::Output {
        (self.src1.call_par(pool.clone()), self.src2.call_par(pool))
    }
}

impl<T: Node> JoinExt for T {}
pub trait JoinExt: Node {
    fn join<Src2>(self, src2: Src2) -> Join<Self, Src2>
    where
        Self: Sized,
        Src2: Node,
    {
        Join::new(self, src2)
    }

    fn par_join<Src2>(self, src2: Src2) -> ParJoin<Self, Src2>
    where
        Self: Sized + ParNode,
        Src2: ParNode,
    {
        ParJoin::new(self, src2)
    }
}
