use crate::NodeExec;

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

impl<Src> NodeExec for Duplicable<Src>
where
    Src: NodeExec,
{
    type Output = Src::Output;

    fn exec(self) -> Self::Output {
        self.src.exec()
    }
}

impl<T: NodeExec> DupeExt for T {}
pub trait DupeExt: NodeExec {
    fn duplicable(self) -> Duplicable<Self>
    where
        Self: Sized + Clone,
    {
        Duplicable { src: self }
    }
}
