use choreo::{
    nodes::{self, AsyncExt, SharedExt, Source, SourceExt, SplitExt, ThenExt},
    GraphNode,
};

#[choreo::graph]
pub async fn add_values(v1: u32, v2: u32, v3: u32) -> u32 {
    println!("waiting for 2 seconds, then adding ({v1} + {v2} + {v3})...");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    v1 + v2 + v3
}

fn test() {
    let src1 = Source::new(5);
    let src2 = Source::new(6);
    let src3 = Source::new(7);
    let builder = AddValuesBuilder::new()
        .i0_node(src1)
        .i1_node(src2)
        .i2_node(src3)
        .build_node();
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // // create splitable source
    // let split_src = 6.source_node().splitable();

    // // create node that adds 3 values
    // let add_node1 = add_values(
    //     8.source_node().asyncify(),
    //     // use the split source twice here
    //     split_src.split().asyncify(),
    //     split_src.split().asyncify(),
    // )
    // // add an intermediary step that prints the resulting value
    // .then(|value| async move {
    //     let value = value.await;
    //     println!("Step Result: {value}");
    //     value
    // })
    // // when a future is produced by a node,
    // // it must first be made shared to be splitable.
    // // this is because splitable requires the output to be clone,
    // // and futures can not usually be cloned by default
    // .shared()
    // .splitable();

    // // create second add values node and split the first into 3 parts
    // let add_node2 = add_values(
    //     // use the first add node twice here
    //     add_node1.split(),
    //     add_node1.split(),
    //     // use the original split source node again
    //     split_src.asyncify(),
    // );

    // // process the last node and get the output
    // let output = add_node2.execute().await;
    // println!("Final Result: {output}");
}
