use crate::GraphNode;

use super::Source;

pub struct Pipeline<Src> {
    src: Src,
}

impl<In, Out, F> From<Source<In, Out, F>> for Pipeline<Source<In, Out, F>> {
    fn from(src: Source<In, Out, F>) -> Self {
        Self { src }
    }
}

impl<In, Out, F> Pipeline<Source<In, Out, F>>
where
    F: Fn(In) -> Out,
{
    pub fn source(action: F) -> Self {
        Source::new(action).into()
    }
}

impl<Src: GraphNode> GraphNode for Pipeline<Src> {
    type In = Src::In;
    type Out = Src::Out;

    fn exec_ctx(&self, ctx: &mut crate::GraphCtx, input: Self::In) -> Self::Out {
        self.src.exec_ctx(ctx, input)
    }
}
