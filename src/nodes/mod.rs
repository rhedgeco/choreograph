pub mod dupe;
pub mod fork;
pub mod future;
pub mod source;
pub mod split;
pub mod then;

pub use dupe::DupeExt;
pub use fork::ForkExt;
pub use future::FutureExt;
pub use source::Source;
pub use split::SplitExt;
pub use then::ThenExt;

// re-export node macros
pub use choreo_macros::merge;
