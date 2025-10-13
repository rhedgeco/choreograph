use std::{
    thread,
    time::{Duration, Instant},
};

use choreograph::{
    Node,
    node::{BranchExt, Task},
};

fn main() {
    let long_task = Task::new(|| {
        // sleep for 1 second
        thread::sleep(Duration::from_secs(1));

        // then return a value
        100
    })
    // make the long task branchable so that branches can be created
    .branchable();

    // create a branch to use the task in two locations
    let task_branch = long_task.branch();

    // create a task that runs both tasks on separate threads
    let final_task = Task::new(|| {
        let value1 = thread::spawn(|| task_branch.execute()).join().unwrap();
        let value2 = long_task.execute();
        value1 + value2
    });

    // execute and measure the final task
    let start_time = Instant::now();
    println!("Executing multithreaded task...");
    let output = final_task.execute();
    let delta = Instant::now().duration_since(start_time).as_millis();
    println!("Calculated output {output} after {delta}ms");
}
