use crate::NodeExec;

pub struct Then<Src, F> {
    action: F,
    src: Src,
}

impl<Src, F, Out> NodeExec for Then<Src, F>
where
    Src: NodeExec,
    F: FnOnce(Src::Output) -> Out,
{
    type Output = Out;

    fn exec(self) -> Self::Output {
        (self.action)(self.src.exec())
    }
}

impl<T: NodeExec> ThenExt for T {}
pub trait ThenExt: NodeExec {
    fn then<F, Out>(self, action: F) -> Then<Self, F>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> Out,
    {
        Then { action, src: self }
    }
}
