use std::iter::Enumerate;

pub struct SkipNth<I> {
    inner: Enumerate<I>,
    skip: usize,
}

impl<I: Iterator> Iterator for SkipNth<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let (index, item) = self.inner.next()?;
        match index == self.skip {
            false => Some(item),
            true => self.next(),
        }
    }
}

impl<I: Iterator> SkipNth<I> {
    pub fn new(iter: I, skip: usize) -> Self {
        Self {
            inner: iter.enumerate(),
            skip,
        }
    }
}

impl<I: Iterator> SkipNthExt for I {}
pub trait SkipNthExt: Iterator {
    fn skip_nth(self, skip: usize) -> SkipNth<Self>
    where
        Self: Sized,
    {
        SkipNth::new(self, skip)
    }
}
