use std::{cell::Cell, rc::Rc};

use crate::Node;

struct SplitInner<Src, L, R, F> {
    src: Cell<Option<(Src, F)>>,
    lhs: Cell<Option<L>>,
    rhs: Cell<Option<R>>,
}

impl<Src, L, R, F> SplitInner<Src, L, R, F>
where
    Src: Node,
    F: FnOnce(Src::Output) -> (L, R),
{
    pub fn split(&self) -> (L, R) {
        match self.src.take() {
            None => unreachable!("cannot split twice"),
            Some((src, action)) => action(src.call()),
        }
    }
}

pub struct Split(());
impl Split {
    pub fn new<Src, L, R, F>(src: Src, action: F) -> (SplitL<Src, L, R, F>, SplitR<Src, L, R, F>)
    where
        Src: Node,
        F: FnOnce(Src::Output) -> (L, R),
    {
        let inner = Rc::new(SplitInner {
            src: Cell::new(Some((src, action))),
            lhs: Cell::new(None),
            rhs: Cell::new(None),
        });

        (
            SplitL {
                inner: inner.clone(),
            },
            SplitR { inner },
        )
    }
}

pub struct SplitL<Src, L, R, F> {
    inner: Rc<SplitInner<Src, L, R, F>>,
}

impl<Src, L, R, F> Node for SplitL<Src, L, R, F>
where
    Src: Node,
    F: FnOnce(Src::Output) -> (L, R),
{
    type Output = L;

    fn call(self) -> Self::Output {
        match self.inner.lhs.take() {
            Some(lhs) => lhs,
            None => {
                let (lhs, rhs) = self.inner.split();
                self.inner.rhs.set(Some(rhs));
                lhs
            }
        }
    }
}

pub struct SplitR<Src, L, R, F> {
    inner: Rc<SplitInner<Src, L, R, F>>,
}

impl<Src, L, R, F> Node for SplitR<Src, L, R, F>
where
    Src: Node,
    F: FnOnce(Src::Output) -> (L, R),
{
    type Output = R;

    fn call(self) -> Self::Output {
        match self.inner.rhs.take() {
            Some(rhs) => rhs,
            None => {
                let (lhs, rhs) = self.inner.split();
                self.inner.lhs.set(Some(lhs));
                rhs
            }
        }
    }
}

impl<T: Node> SplitExt for T {}
pub trait SplitExt: Node {
    fn split<L, R, F>(self, action: F) -> (SplitL<Self, L, R, F>, SplitR<Self, L, R, F>)
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> (L, R),
    {
        Split::new(self, action)
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::Source;

    use super::*;

    #[test]
    fn simple_split() {
        let source = Source::new("helloworld");
        let (lhs, rhs) = source.split(|str| str.split_at(5));
        assert_eq!(lhs.call(), "hello");
        assert_eq!(rhs.call(), "world");
    }
}
