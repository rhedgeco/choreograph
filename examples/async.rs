use std::time::Duration;

use choreograph::{
    Node,
    node::{BranchExt, FutureExt, SharedExt, Task},
};
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    // async nodes can be created by calling `future()` on a sync node
    let short_task = Task::wrap(50).future();

    // or they can be created by directly wrapping a future
    let long_task = Task::wrap(async {
        // sleep task for 1 second
        tokio::time::sleep(Duration::from_secs(1)).await;

        // then return a value
        100
    })
    // to make this async task branchable it will have to be `shared`.
    .shared()
    // this is because `branchable` requires the output to be `Clone`,
    // and most futures do not implement `Clone` by default.
    .branchable();

    // create a branch to use the task in two locations
    let task_branch = long_task.branch();

    // create a final task that runs all other tasks
    let final_task = Task::wrap(async {
        let value1 = long_task.execute().await;
        let value2 = task_branch.execute().await;
        let value3 = short_task.execute().await;
        value1 + value2 + value3
    });

    // execute and measure the final task
    let start_time = Instant::now();
    println!("Executing async task...");
    let output = final_task.execute().await;
    let delta = Instant::now().duration_since(start_time).as_millis();
    println!("Calculated output {output} after {delta}ms");
}
