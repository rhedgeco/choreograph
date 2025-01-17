use std::sync::atomic::{AtomicU32, Ordering};

use crate::GraphExecutor;

use super::{BranchExt, CacheExt, Entry, JoinExt};

#[test]
fn simple_join() {
    // create test inputs and calculate expected output
    const INPUT: u32 = 50;
    const INITIAL_ADD: u32 = 100;
    const ADD_AMOUNT: u32 = 20;
    const SUB_AMOUNT: u32 = 10;
    const OUTPUT: u32 =
        ((INPUT + INITIAL_ADD + ADD_AMOUNT) + (INPUT + INITIAL_ADD - SUB_AMOUNT)) / 2;

    // build the tasks that branch off from root then join at end
    let root_task = Entry::new(|v: u32| v + 100).cached();
    let add_task = root_task.branch(|v| v + 20);
    let sub_task = root_task.branch(|v| v - 10);
    let join_task = add_task.join(sub_task, |(v1, v2)| (v1 + v2) / 2);

    // execute the task and check if it is valid
    let output = join_task.execute(INPUT);
    assert_eq!(output, OUTPUT);
}

#[test]
fn uncached_calculations() {
    // create the test input and calculate the expected output
    const INPUT: u32 = 10;
    const OUTPUT: u32 = (INPUT + 1) + (INPUT + 2) + (INPUT + 3);

    // build a simple graph root with no caching
    let root_task = Entry::new(|v: u32| {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        v + COUNTER.fetch_add(1, Ordering::Relaxed)
    });

    // branch off the always task 3 times
    let branch1 = root_task.branch(|v| v);
    let branch2 = root_task.branch(|v| v);
    let branch3 = root_task.branch(|v| v);

    // join all three branches
    let join1 = branch1.join(branch2, |(v1, v2)| v1 + v2).cached();
    let join2 = join1.join(branch3, |(v1, v2)| v1 + v2);

    // execute the graph and test that the output is correct
    let output = join2.execute(INPUT);
    assert_eq!(output, OUTPUT);
}
