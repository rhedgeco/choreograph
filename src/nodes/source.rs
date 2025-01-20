use std::marker::PhantomData;

use crate::{GraphCtx, GraphNode};

pub struct Source<In, Out, F> {
    _types: PhantomData<fn(In) -> Out>,
    action: F,
}

impl<In, Out, F> Source<In, Out, F>
where
    F: Fn(In) -> Out,
{
    pub const fn new(action: F) -> Self {
        Self {
            _types: PhantomData,
            action,
        }
    }
}

impl<In, Out, F> GraphNode for Source<In, Out, F>
where
    F: Fn(In) -> Out,
{
    type In = In;
    type Out = Out;

    fn exec_ctx(&self, _: &mut GraphCtx, input: Self::In) -> Self::Out {
        (self.action)(input)
    }
}
