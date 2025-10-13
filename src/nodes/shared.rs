use futures::{FutureExt, future};

use crate::Node;

pub struct Shared<Src> {
    src: Src,
}

impl<Src> Node for Shared<Src>
where
    Src: Node,
    Src::Output: Future,
    <Src::Output as Future>::Output: Clone,
{
    type Output = future::Shared<Src::Output>;

    fn execute(self) -> Self::Output {
        self.src.execute().shared()
    }
}

impl<T: Node> SharedExt for T {}
pub trait SharedExt: Node {
    fn shared(self) -> Shared<Self>
    where
        Self: Sized,
        Self::Output: Future,
        <Self::Output as Future>::Output: Clone,
    {
        Shared { src: self }
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::Task;

    use super::*;

    #[tokio::test]
    async fn executes_correctly() {
        let task = Task::wrap(async { 100 }).shared();
        let future1 = task.execute();
        let future2 = future1.clone();
        let value1 = future1.await;
        let value2 = future2.await;
        assert_eq!(value1, 100);
        assert_eq!(value2, 100);
    }
}
