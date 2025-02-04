use choreo::{
    nodes::{Action, FutureExt, SyncExt, ThenExt},
    Node,
};
use futures::join;

// example async function
async fn add_values(s1: u32, s2: u32, s3: u32, s4: u32) -> u32 {
    s1 + s2 + s3 + s4
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // create source nodes
    let source1 = Action::new(|| 16);
    let source2 = Action::new(|| 28);

    // create branching async source nodes
    let source3 = Action::new(|| 11).awaitable().synced();
    let source4 = source3.clone().then(|v| async { v.await + 3 });

    // merge the nodes to be used with `add_values`
    let merge = Action::new(|| async {
        let (s1, s2) = (source1.execute(), source2.execute());
        let (s3, s4) = join!(source3.execute(), source4.execute());
        add_values(s1, s2, s3, s4).await
    });

    // execute and await the output
    let output = merge.execute().await;
    println!("{output}");
}
