use std::marker::PhantomData;

use crate::{GraphCtx, GraphNode};

/// A simple [`GraphNode`] that takes input, but also depends on other data or actions.
///
/// # Generics
/// - `In`: The input type consumed by this action.
/// - `Out`: The output type produced by this action.
/// - `Deps`: The dependencies of this action.
/// - `F`: The function implementation for this action.
pub struct Action<In, Out, Deps, F> {
    _types: PhantomData<fn(In) -> Out>,
    deps: Deps,
    action: F,
}

impl<In, Out, Deps, F> Action<In, Out, Deps, F>
where
    F: Fn(&Deps, In) -> Out,
{
    pub const fn new(deps: Deps, action: F) -> Self {
        Self {
            _types: PhantomData,
            action,
            deps,
        }
    }
}

impl<In, Out, Deps, F> GraphNode for Action<In, Out, Deps, F>
where
    F: Fn(&Deps, In) -> Out,
{
    type In = In;
    type Out = Out;

    fn exec_ctx(&self, _: &mut GraphCtx, input: Self::In) -> Self::Out {
        (self.action)(&self.deps, input)
    }
}
