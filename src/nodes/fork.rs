use std::{
    cell::{Cell, OnceCell},
    rc::Rc,
};

use crate::NodeExec;

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

impl<Out, Src> NodeExec for Forkable<Out, Src>
where
    Out: Clone,
    Src: NodeExec<Output = Out>,
{
    type Output = Out;

    fn exec(self) -> Self::Output {
        let output = self.inner.out.get_or_init(|| match self.inner.src.take() {
            None => unreachable!("cannot init once cell twice"),
            Some(src) => src.exec(),
        });

        output.clone()
    }
}

impl<T: NodeExec> ForkExt for T {}
pub trait ForkExt: NodeExec {
    fn forkable<Out>(self) -> Forkable<Out, Self>
    where
        Out: Clone,
        Self: NodeExec<Output = Out> + Sized,
    {
        Forkable {
            inner: Rc::new(Inner {
                src: Cell::new(Some(self)),
                out: OnceCell::new(),
            }),
        }
    }
}
