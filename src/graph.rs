use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

// re-export graph builder macro
pub use choreo_macros::graph_builder as builder;

/// An extension trait for building a [`GraphCtx`] and executing the [`Graph`] all the way through
pub trait GraphExecutor: Graph {
    /// Builds a [`GraphCtx`] and executes the [`Graph`] all the way through
    fn execute(&self, input: Self::Input) -> Self::Output {
        self.execute_with_ctx(&mut GraphCtx::new(), input)
    }
}
impl<T: Graph> GraphExecutor for T {}

/// A trait that defines the structure of a graph
///
/// GraphNodes have an Input type, Output type, and an function that executes the graph
pub trait Graph: Copy {
    type Input;
    type Output;
    fn execute_with_ctx(&self, ctx: &mut GraphCtx, input: Self::Input) -> Self::Output;
}

/// A context manager for [`Graph`] executions
pub struct GraphCtx {
    items: HashMap<TypeId, Box<dyn Any>>,
}

impl GraphCtx {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
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
