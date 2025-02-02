use std::{cell::UnsafeCell, hint::unreachable_unchecked, mem::ManuallyDrop, sync::Arc};

use parking_lot::{Once, OnceState};

use crate::Node;

/// A node that runs once, and syncs its output across all forked copies.
///
/// A synced node can be created by calling [synced](SyncExt::synced).
/// For a node to be able to be synced, its output must implement [`Clone`].
pub struct Synced<Src, Out> {
    inner: Arc<Inner<Src, Out>>,
}

impl<Src, Out> Clone for Synced<Src, Out> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Src, Out> Node for Synced<Src, Out>
where
    Src: Node<Output = Out>,
    Out: Clone,
{
    type Output = Out;

    fn exec(self) -> Self::Output {
        self.inner.exec()
    }
}

impl<T: Node> SyncExt for T {}
pub trait SyncExt: Node {
    /// Wraps the current node in [`Synced`] node.
    fn synced<Out>(self) -> Synced<Self, Out>
    where
        Self: SyncExt<Output = Out> + Sized,
        Out: Clone,
    {
        Synced {
            inner: Arc::new(Inner::new(self)),
        }
    }
}

union Data<Src, Out> {
    src: ManuallyDrop<Src>,
    out: ManuallyDrop<Out>,
}

struct Inner<Src, Out> {
    data: UnsafeCell<Data<Src, Out>>,
    once: Once,
}

impl<Src, Out> Drop for Inner<Src, Out> {
    fn drop(&mut self) {
        match self.once.state() {
            // SAFETY: if the `once` has not been called yet, then only the `src` state is valid.
            OnceState::New => unsafe { ManuallyDrop::drop(&mut self.data.get_mut().src) },
            // SAFETY: if the `once` is completed, then only the `out` state is valid.
            OnceState::Done => unsafe { ManuallyDrop::drop(&mut self.data.get_mut().out) },
            // SAFETY: impossible to reach, with exclusive access there is no other thread acessing this.
            OnceState::InProgress => unsafe { unreachable_unchecked() },
            // Do nothing. If `once` is poisoned, then neither the `src` or `out` states are valid.
            OnceState::Poisoned => {}
        }
    }
}

impl<Src, Out> Inner<Src, Out>
where
    Src: Node<Output = Out>,
    Out: Clone,
{
    pub fn new(src: Src) -> Self {
        Self {
            data: UnsafeCell::new(Data {
                src: ManuallyDrop::new(src),
            }),
            once: Once::new(),
        }
    }

    pub fn exec(&self) -> Out {
        // implementation is almost identical to `LazyLock::deref`.
        self.once.call_once(|| {
            // SAFETY: `call_once` only runs this closure once, ever.
            let data = unsafe { &mut *self.data.get() };
            let src = unsafe { ManuallyDrop::take(&mut data.src) };
            let out = src.exec();
            data.out = ManuallyDrop::new(out);
        });

        // SAFETY:
        // There are four possible scenarios:
        // * `src` was executed and initialized `out`.
        // * `src` was executed and panicked, so this point is never reached.
        // * `src` was not executed, but a previous call initialized `out`.
        // * `src` was not executed because the Once is poisoned, so this point is never reached.
        // So `out` has definitely been initialized and will not be modified again.
        unsafe { &*(*self.data.get()).out }.clone()
    }
}

// only implement sync if both `Src` and `Out` are Send.
// We never create a `&Src` or `&Out` from a `&Inner<Src, Out>`,
// so it is fine to not enforce `Sync`.
unsafe impl<Src: Send, Out: Send> Sync for Inner<Src, Out> {}
// auto-derived `Send` impl is OK.
