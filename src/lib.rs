pub mod graph;
pub mod nodes;

pub use graph::GraphNode;

// extern export self for proc macros
pub extern crate self as choreo;
