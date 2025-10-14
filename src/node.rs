pub mod branch;
pub mod future;
pub mod shared;
pub mod task;
pub mod then;

pub use branch::{Branch, BranchExt};
pub use future::{Async, AsyncExt};
pub use shared::{Shared, SharedExt};
pub use task::Task;
pub use then::{Then, ThenExt};

pub trait Node {
    type Output;
    fn execute(self) -> Self::Output;
}
