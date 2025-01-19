use std::marker::PhantomData;

use crate::{GraphCtx, GraphNode};

pub struct Depend<In, Out, Deps, F> {
    _types: PhantomData<fn(In) -> Out>,
    deps: Deps,
    action: F,
}

impl<In, Out, Deps, F> Depend<In, Out, Deps, F>
where
    F: Fn(&Deps, In) -> Out,
{
    pub fn new(deps: Deps, action: F) -> Self {
        Self {
            _types: PhantomData,
            action,
            deps,
        }
    }
}

impl<In, Out, Deps, F> GraphNode for Depend<In, Out, Deps, F>
where
    F: Fn(&Deps, In) -> Out,
{
    type In = In;
    type Out = Out;

    fn exec_ctx(&self, _: &mut GraphCtx, input: Self::In) -> Self::Out {
        (self.action)(&self.deps, input)
    }
}
