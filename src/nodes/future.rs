use std::future;

use crate::Node;

pub struct Future<Src> {
    src: Src,
}

impl<Src> Node for Future<Src>
where
    Src: Node,
{
    type Output = future::Ready<Src::Output>;

    fn execute(self) -> Self::Output {
        future::ready(self.src.execute())
    }
}

impl<T: Node> FutureExt for T {}
pub trait FutureExt: Node {
    fn future(self) -> Future<Self>
    where
        Self: Sized,
    {
        Future { src: self }
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::Task;

    use super::*;

    #[tokio::test]
    async fn executes_correctly() {
        let task = Task::wrap(100).future();
        let value = task.execute().await;
        assert_eq!(value, 100);
    }
}
