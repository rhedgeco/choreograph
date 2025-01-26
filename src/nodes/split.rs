use std::{
    cell::{Cell, OnceCell},
    rc::Rc,
};

use crate::Node;

struct Inner<Out, Src> {
    src: Cell<Option<Src>>,
    out: OnceCell<Out>,
}

pub struct Split<Out, Src> {
    inner: Rc<Inner<Out, Src>>,
}

impl<Out, Src> Split<Out, Src>
where
    Self: Node,
{
    pub fn new(src: Src) -> Self {
        Self {
            inner: Rc::new(Inner {
                src: Cell::new(Some(src)),
                out: OnceCell::new(),
            }),
        }
    }

    pub fn split(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Out, Src> Node for Split<Out, Src>
where
    Out: Clone,
    Src: Node<Output = Out>,
{
    type Output = Out;

    fn call(self) -> Self::Output {
        // initialize the
        let output = self.inner.out.get_or_init(|| match self.inner.src.take() {
            Some(src) => src.call(),
            None => unreachable!(),
        });

        output.clone()
    }
}

impl<T: Node> SplitExt for T {}
pub trait SplitExt: Node {
    fn splittable<Out>(self) -> Split<Out, Self>
    where
        Out: Clone,
        Self: Node<Output = Out> + Sized,
    {
        Split::new(self)
    }
}
