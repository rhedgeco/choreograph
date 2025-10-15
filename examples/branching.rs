use std::{
    thread,
    time::{Duration, Instant},
};

use choreograph::{Task, task::BranchExt};

fn main() {
    let long_task = (|| {
        // sleep for 1 second
        thread::sleep(Duration::from_secs(1));

        // then return a value
        100
    })
    // make the long task branchable
    .branchable();

    // branched tasks ensures that every branch task gets the exact same output.
    // this also means that the body of the task is only executed once and the output is shared.
    let long_branch = long_task.branch();

    // create another task that execuets both long task branches
    let final_task = || {
        let value1 = long_branch.execute();
        let value2 = long_task.execute();
        value1 + value2
    };

    // execute and measure the final task
    let start_time = Instant::now();
    println!("Executing task graph...");
    let output = final_task.execute();
    let delta = Instant::now().duration_since(start_time).as_millis();
    println!("Calculated output {output} after {delta}ms");
}
