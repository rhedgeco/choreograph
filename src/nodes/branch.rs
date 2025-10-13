use std::{
    cell::UnsafeCell,
    sync::{Arc, Once},
};

use crate::Node;

struct Inner<Src: Node> {
    output: UnsafeCell<Option<Src::Output>>,
    src: UnsafeCell<Option<Src>>,
    once: Once,
}

// SAFETY:
// The `UnsafeCell` is the structure limiting `Sync` from being implemented automatically.
// `UnsafeCell` does not implement Sync because it can be used to access a mutable pointer from multiple threads.
// The implementation of `Branch` adheres to these rules and only uses locked (with `Once::call_once`), or non-exclusive access.
unsafe impl<Src: Node> Sync for Inner<Src> {}

pub struct Branch<Src: Node> {
    inner: Arc<Inner<Src>>,
}

impl<Src: Node> Branch<Src> {
    pub fn branch(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Src: Node> Node for Branch<Src>
where
    Src::Output: Clone,
{
    type Output = Src::Output;

    fn execute(mut self) -> Self::Output {
        // initialize the output value
        self.inner.once.call_once(|| {
            // get a mutable pointer to the source node
            let src_ptr = self.inner.src.get();

            // SAFETY:
            // Since this executes inside the `call_once` function,
            // we are guaranteed that we can get the src as mutable.
            let src_option = unsafe { &mut *src_ptr }.take();

            // SAFETY:
            // We are also guaranteed to have `Some` node here,
            // since the node is only taken out inside the `call_once` function,
            // and the node is always assigned as `Some` when it is created.
            let src = unsafe { src_option.unwrap_unchecked() };

            // get a mutable pointer to the output storage location
            let output_ptr = self.inner.output.get();

            // SAFETY:
            // Since this executes inside the `call_once` function,
            // we can guarantee that this mutable assignment is valid.
            unsafe { *output_ptr = Some(src.execute()) };
        });

        // try to get the inner node as mutable
        // this saves one clone of the output when the node is called for the last time
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            // SAFETY:
            // We are also guaranteed to have `Some` output here.
            // The only time the output is taken, is when the arc is accessed as mutable.
            // An arc is only able to be mutably accessed when there is only one reference.
            // The arc is then dropped at the end of this function as the function consumes `self`.
            return unsafe { inner.output.get_mut().take().unwrap_unchecked() };
        }

        // If we get here, then we were not able to access the arc as mutable.
        // This means we should just clone and return the stored output value.

        // Get a pointer to the stored output value
        let output_ptr = self.inner.output.get();

        // SAFETY:
        // The output ptr is only used in a non-exclusive manner here.
        // The output pointer is only ever used in a non-exclusive manner after the `call_once` call.
        let output_option = unsafe { &*output_ptr }.clone();

        // SAFETY:
        // This branch can never be reached.
        // The output is guaranteed to be initialized to `Some` after the `call_once` call.
        // The output is never taken out, unless it is taken in the `Arc::get_mut` context above.
        // When the `Arc::get_mut` context completes it early returns, and guarantees that it was the last node.
        unsafe { output_option.unwrap_unchecked() }
    }
}

impl<T: Node> BranchExt for T {}
pub trait BranchExt: Node {
    fn branchable(self) -> Branch<Self>
    where
        Self: Sized,
        Self::Output: Clone,
    {
        Branch {
            inner: Arc::new(Inner {
                output: UnsafeCell::new(None),
                src: UnsafeCell::new(Some(self)),
                once: Once::new(),
            }),
        }
    }
}
