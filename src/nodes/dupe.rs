use crate::GraphNode;

pub struct Duplicable<Src> {
    src: Src,
}

impl<Src> Duplicable<Src> {
    pub fn duplicate(&self) -> Self
    where
        Src: Clone,
    {
        Self {
            src: self.src.clone(),
        }
    }
}

impl<Src> GraphNode for Duplicable<Src>
where
    Src: GraphNode,
{
    type Output = Src::Output;

    fn execute(self) -> Self::Output {
        self.src.execute()
    }
}

impl<T: GraphNode> DupeExt for T {}
pub trait DupeExt: GraphNode {
    fn duplicable(self) -> Duplicable<Self>
    where
        Self: Sized + Clone,
    {
        Duplicable { src: self }
    }
}
