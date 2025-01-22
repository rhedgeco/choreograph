use choreo::{
    nodes::{AsyncExt, SharedExt, Source, SplitExt, ThenExt},
    GraphNode,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // create input sources
    let src_node1 = Source::new(8);
    // make one of the sources splitable
    let src_node2 = Source::new(6).splitable();

    // create add values node
    let add_node1 = custom_nodes::add_values(
        src_node1.asyncify(),
        // use the second source twice here
        src_node2.split().asyncify(),
        src_node2.split().asyncify(),
    )
    // add an intermediary step that prints the resulting value
    .then(|value| async move {
        let value = value.await;
        println!("Step Result: {value}");
        value
    })
    // when a future is produced by a node,
    // it must first be made shared to be splitable.
    // this is because splitable requires the output to be clone,
    // and futures can not usually be cloned by default
    .shared()
    .splitable();

    // create second add values node and split the first into 3 parts
    let add_node2 = custom_nodes::add_values(
        // use the first add node twice here
        add_node1.split(),
        add_node1.split(),
        // use the original source2 node again
        src_node2.split().asyncify(),
    );

    // process the last node and get the output
    let output = add_node2.execute().await;
    println!("Final Result: {output}");
}

/// isolated scope so that imports are not depended on
mod custom_nodes {
    /// This is an example of what the procedural macro would generate.
    /// The original `add_values` async function is wrapped by a builder function.
    /// The wrapper instead takes in graph nodes that produce the required input values.
    /// It then joins all the input nodes together and creates an action that executes the original function.
    pub fn add_values(
        v1: impl ::choreo::GraphNode<Output = impl ::std::future::Future<Output = u32>>,
        v2: impl ::choreo::GraphNode<Output = impl ::std::future::Future<Output = u32>>,
        v3: impl ::choreo::GraphNode<Output = impl ::std::future::Future<Output = u32>>,
    ) -> impl ::choreo::GraphNode<Output = impl ::std::future::Future<Output = u32>> {
        async fn add_values(
            v1: impl ::std::future::Future<Output = u32>,
            v2: impl ::std::future::Future<Output = u32>,
            v3: impl ::std::future::Future<Output = u32>,
        ) -> u32 {
            // inner function here is written by the user
            let (v1, v2, v3) = futures::join!(v1, v2, v3);
            println!("waiting for 2 seconds, then adding ({v1} + {v2} + {v3})...");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            v1 + v2 + v3
        }

        use ::choreo::nodes::{JoinExt, ThenExt};
        v1.join(v2)
            .join(v3)
            .then(|((v1, v2), v3)| add_values(v1, v2, v3))
    }
}
