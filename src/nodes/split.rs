use std::{
    cell::{Cell, OnceCell},
    rc::Rc,
};

use crate::GraphNode;

struct Inner<Out, Src> {
    src: Cell<Option<Src>>,
    out: OnceCell<Out>,
}

pub struct Split<Out, Src> {
    inner: Rc<Inner<Out, Src>>,
}

impl<Out, Src> Split<Out, Src> {
    pub fn split(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Out, Src> GraphNode for Split<Out, Src>
where
    Out: Clone,
    Src: GraphNode<Output = Out>,
{
    type Output = Out;

    fn execute(self) -> Self::Output {
        self.inner
            .out
            .get_or_init(|| match self.inner.src.take() {
                Some(src) => src.execute(),
                None => unreachable!(),
            })
            .clone()
    }
}

impl<T: GraphNode> SplitExt for T {}
pub trait SplitExt: GraphNode {
    fn splitable<Out>(self) -> Split<Out, Self>
    where
        Out: Clone,
        Self: GraphNode<Output = Out> + Sized,
    {
        Split {
            inner: Rc::new(Inner {
                src: Cell::new(Some(self)),
                out: OnceCell::new(),
            }),
        }
    }
}
