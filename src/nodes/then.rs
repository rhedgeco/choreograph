use crate::Node;

pub struct Then<Src, F> {
    action: F,
    src: Src,
}

impl<Src, F> Then<Src, F> {
    pub fn new<Out>(src: Src, action: F) -> Self
    where
        Src: Node,
        F: FnOnce(Src::Output) -> Out,
    {
        Self { action, src }
    }
}

impl<Src, F, Out> Node for Then<Src, F>
where
    Src: Node,
    F: FnOnce(Src::Output) -> Out,
{
    type Output = Out;

    fn call(self) -> Self::Output {
        (self.action)(self.src.call())
    }
}

impl<T: Node> ThenExt for T {}
pub trait ThenExt: Node {
    fn then<F, Out>(self, action: F) -> Then<Self, F>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> Out,
    {
        Then::new(self, action)
    }
}
