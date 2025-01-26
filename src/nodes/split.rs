use std::{
    cell::{Cell, OnceCell},
    rc::Rc,
    sync::{Arc, Mutex, OnceLock},
};

use static_assertions::assert_impl_all;

use crate::{nodes::Source, pool::ThreadPool, Node, ParNode};

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

struct ParInner<Out, Src> {
    src: Mutex<Option<Src>>,
    out: OnceLock<Out>,
}

pub struct ParSplit<Out, Src> {
    inner: Arc<ParInner<Out, Src>>,
}
assert_impl_all!(ParSplit<u32, Source<u32>>: Send, Sync);

impl<Out, Src> ParSplit<Out, Src>
where
    Self: ParNode,
{
    pub fn new(src: Src) -> Self {
        Self {
            inner: Arc::new(ParInner {
                src: Mutex::new(Some(src)),
                out: OnceLock::new(),
            }),
        }
    }

    pub fn split(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Out, Src> ParNode for ParSplit<Out, Src>
where
    Out: Clone,
    Src: ParNode<Output = Out>,
{
    type Output = Out;

    fn call_par(self, pool: Arc<impl ThreadPool>) -> Self::Output {
        let output = self.inner.out.get_or_init(|| {
            let mut lock = self.inner.src.lock().unwrap();
            match lock.take() {
                Some(src) => src.call_par(pool),
                None => unreachable!(),
            }
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

    fn par_splittable<Out>(self) -> ParSplit<Out, Self>
    where
        Out: Clone,
        Self: ParNode<Output = Out> + Sized,
    {
        ParSplit::new(self)
    }
}
