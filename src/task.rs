pub mod branch;
pub mod then;

pub use branch::{Branch, BranchExt};
pub use then::{Then, ThenExt};

pub trait Task {
    type Output;
    fn execute(self) -> Self::Output;
}

impl<Out, T: FnOnce() -> Out> Task for T {
    type Output = Out;
    fn execute(self) -> Self::Output {
        (self)()
    }
}
