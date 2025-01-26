use std::marker::PhantomData;

use crate::Node;

pub struct Then<Out, Src, F> {
    _types: PhantomData<fn() -> Out>,
    action: F,
    src: Src,
}

impl<Out, Src, F> Then<Out, Src, F>
where
    Self: Node,
{
    pub fn new(src: Src, action: F) -> Self {
        Self {
            _types: PhantomData,
            action,
            src,
        }
    }
}

impl<Out, Src, F> Node for Then<Out, Src, F>
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
    fn then<Out, F>(self, action: F) -> Then<Out, Self, F>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> Out,
    {
        Then::new(self, action)
    }
}
