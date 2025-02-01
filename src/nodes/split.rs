use std::{cell::Cell, rc::Rc};

use crate::NodeExec;

struct SplitInner<Src, L, R, F> {
    src: Cell<Option<(Src, F)>>,
    lhs: Cell<Option<L>>,
    rhs: Cell<Option<R>>,
}

impl<Src, L, R, F> SplitInner<Src, L, R, F>
where
    Src: NodeExec,
    F: FnOnce(Src::Output) -> (L, R),
{
    pub fn split(&self) -> (L, R) {
        match self.src.take() {
            None => unreachable!("cannot split twice"),
            Some((src, action)) => action(src.exec()),
        }
    }
}

pub struct SplitL<Src, L, R, F> {
    inner: Rc<SplitInner<Src, L, R, F>>,
}

impl<Src, L, R, F> NodeExec for SplitL<Src, L, R, F>
where
    Src: NodeExec,
    F: FnOnce(Src::Output) -> (L, R),
{
    type Output = L;

    fn exec(self) -> Self::Output {
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

impl<Src, L, R, F> NodeExec for SplitR<Src, L, R, F>
where
    Src: NodeExec,
    F: FnOnce(Src::Output) -> (L, R),
{
    type Output = R;

    fn exec(self) -> Self::Output {
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

impl<T: NodeExec> SplitExt for T {}
pub trait SplitExt: NodeExec {
    fn split<L, R, F>(self, action: F) -> (SplitL<Self, L, R, F>, SplitR<Self, L, R, F>)
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> (L, R),
    {
        let inner = Rc::new(SplitInner {
            src: Cell::new(Some((self, action))),
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

#[cfg(test)]
mod tests {
    use crate::Node;

    use super::*;

    #[test]
    fn simple_split() {
        let data = Node::new(|| "helloworld");
        let (lhs, rhs) = data.split(|str| str.split_at(5));
        assert_eq!(lhs.exec(), "hello");
        assert_eq!(rhs.exec(), "world");
    }
}
