use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

pub trait GraphNode {
    type In;
    type Out;
    fn exec_ctx(&self, ctx: &mut GraphCtx, input: Self::In) -> Self::Out;
}

/// An unique identifier used for identifying a [`GraphCtx`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphId(u64);

impl GraphId {
    /// Generates a new unique identifier.
    pub fn unique() -> Self {
        static GEN: AtomicU64 = AtomicU64::new(0);
        Self(GEN.fetch_add(1, Ordering::Relaxed))
    }
}

/// A context manager for [`Graph`] executions
pub struct GraphCtx {
    items: HashMap<TypeId, Box<dyn Any>>,
    id: GraphId,
}

impl GraphCtx {
    /// Builds a graph context and executes a [`GraphNode`].
    ///
    /// The context only lives for the length of the execution.
    pub fn execute<T: GraphNode>(node: &mut T, input: T::In) -> T::Out {
        node.exec_ctx(
            &mut GraphCtx {
                items: HashMap::new(),
                id: GraphId::unique(),
            },
            input,
        )
    }

    /// Returns the [`CtxId`] associated with this graph context.
    ///
    /// Every context is guaranteed to have a seperate and unique id.
    pub fn id(&self) -> GraphId {
        self.id
    }

    /// Returns a reference to the item of type `T` stored in this context.
    ///
    /// Returns `None` if no item was found.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        let id = TypeId::of::<T>();
        let item = self.items.get(&id)?;
        Some(item.downcast_ref::<T>().unwrap())
    }

    /// Returns a mutable reference to the item of type `T` stored in this context.
    ///
    /// Returns `None` if no item was found.
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let id = TypeId::of::<T>();
        let item = self.items.get_mut(&id)?;
        Some(item.downcast_mut::<T>().unwrap())
    }

    /// Returns a mutable reference the item of type `T` stored in this context.
    ///
    /// If no item was found, it is constructed using the [`Default`] implementation.
    pub fn get_or_default<T: Default + 'static>(&mut self) -> &mut T {
        self.get_or_insert(T::default)
    }

    /// Returns a mutable reference the item of type `T` stored in this context.
    ///
    /// If no item was found, it is constructed using the `f` callback.
    pub fn get_or_insert<T: 'static>(&mut self, f: impl FnOnce() -> T) -> &mut T {
        let id = TypeId::of::<T>();
        use std::collections::hash_map::Entry as E;
        match self.items.entry(id) {
            E::Occupied(entry) => entry.into_mut(),
            E::Vacant(entry) => entry.insert(Box::new(f())),
        }
        .downcast_mut::<T>()
        .unwrap()
    }
}
