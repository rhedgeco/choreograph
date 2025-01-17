use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

use derive_where::derive_where;

use super::{Graph, GraphCtx};

/// An extension trait that allows for caching a graph output
pub trait CacheExt: Graph {
    /// Caches the wrapped items output when used during graph execution
    fn cached(self) -> Cache<Self>
    where
        Self::Output: Clone + 'static,
    {
        Cache {
            source: self,
            id: CacheId::new(),
        }
    }
}
impl<T: Graph> CacheExt for T {}

/// A graph node that caches the output of the contained graph
#[derive(Debug, Clone, Copy)]
pub struct Cache<Source> {
    source: Source,
    id: CacheId,
}

impl<Source: Graph> Graph for Cache<Source>
where
    Source::Output: Clone + 'static,
{
    type Input = Source::Input;
    type Output = Source::Output;

    fn execute_with_ctx(&self, ctx: &mut GraphCtx, input: Self::Input) -> Self::Output {
        let cache = ctx.get_or_default::<OutputCache<Self::Output>>();
        if let Some(output) = cache.values.get(&self.id) {
            return output.clone();
        }

        let output = self.source.execute_with_ctx(ctx, input);
        let cache = ctx.get_or_default::<OutputCache<Self::Output>>();
        cache.values.entry(self.id).or_insert(output).clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CacheId(u64);

impl CacheId {
    pub fn new() -> Self {
        static GEN: AtomicU64 = AtomicU64::new(0);
        Self(GEN.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive_where(Default)]
struct OutputCache<T> {
    values: HashMap<CacheId, T>,
}
