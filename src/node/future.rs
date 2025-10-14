use std::future;

use crate::Node;

pub struct Async<Src> {
    src: Src,
}

impl<Src> Node for Async<Src>
where
    Src: Node,
{
    type Output = future::Ready<Src::Output>;

    fn resolve(self) -> Self::Output {
        future::ready(self.src.resolve())
    }
}

impl<T: Node> AsyncExt for T {}
pub trait AsyncExt: Node {
    fn future(self) -> Async<Self>
    where
        Self: Sized,
    {
        Async { src: self }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Task;

    use super::*;

    #[tokio::test]
    async fn executes_correctly() {
        let task = Task::wrap(100).future();
        let value = task.resolve().await;
        assert_eq!(value, 100);
    }
}
