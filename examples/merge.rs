use choreo::{nodes::FutureExt, Node, NodeExec};
use futures::join;

// example async function
async fn add_values(s1: u32, s2: u32, s3: u32, s4: u32) -> u32 {
    s1 + s2 + s3 + s4
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // create source nodes, some async and some not
    let source1 = Node::new(|| 15);
    let source2 = Node::new(|| 28).future();
    let source3 = Node::new(|| 17);
    let source4 = Node::new(|| 9).future();

    // merge the nodes to be used with `add_values`
    let merge = Node::new(|| async {
        let (s1, s3) = (source1.exec(), source3.exec());
        let (s2, s4) = join!(source2.exec(), source4.exec());
        add_values(s1, s2, s3, s4).await
    });

    // execute and await the output
    let output = merge.exec().await;
    println!("{output}");
}
