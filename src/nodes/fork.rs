use std::{
    cell::{Cell, OnceCell},
    rc::Rc,
};

use crate::Node;

struct Inner<Out, Src> {
    src: Cell<Option<Src>>,
    out: OnceCell<Out>,
}

pub struct Forkable<Out, Src> {
    inner: Rc<Inner<Out, Src>>,
}

impl<Out, Src> Forkable<Out, Src> {
    pub fn fork(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Out, Src> Node for Forkable<Out, Src>
where
    Out: Clone,
    Src: Node<Output = Out>,
{
    type Output = Out;

    fn call(self) -> Self::Output {
        let output = self.inner.out.get_or_init(|| match self.inner.src.take() {
            None => unreachable!("cannot init once cell twice"),
            Some(src) => src.call(),
        });

        output.clone()
    }
}

impl<T: Node> ForkExt for T {}
pub trait ForkExt: Node {
    fn forkable<Out>(self) -> Forkable<Out, Self>
    where
        Out: Clone,
        Self: Node<Output = Out> + Sized,
    {
        Forkable {
            inner: Rc::new(Inner {
                src: Cell::new(Some(self)),
                out: OnceCell::new(),
            }),
        }
    }
}
