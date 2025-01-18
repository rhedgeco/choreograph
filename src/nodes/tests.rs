use std::sync::atomic::{AtomicU32, Ordering};

use crate::{graph, GraphExecutor};

use super::{BranchExt, CacheExt, Entry, JoinExt, SharedExt};

#[test]
fn simple_join() {
    // create test inputs and calculate expected output
    const INPUT: u32 = 50;
    const ADD_AMOUNT: u32 = 20;
    const SUB_AMOUNT: u32 = 10;
    const OUTPUT: u32 = (INPUT + ADD_AMOUNT + INPUT - SUB_AMOUNT) / 2;

    // build the tasks that branch off from root then join at end
    let entry = Entry::new(|v: u32| v).cached();
    let add_branch = entry.branch(|v| v + 20);
    let sub_branch = entry.branch(|v| v - 10);
    let join = add_branch.join(sub_branch, |v1, v2| (v1 + v2) / 2);

    // execute the task and check if it is valid
    let output = join.execute(INPUT);
    assert_eq!(output, OUTPUT);
}

#[test]
fn uncached_calculations() {
    // create test input and calculate expected output
    const INPUT: u32 = 10;
    const OUTPUT: u32 = (INPUT + 1) + (INPUT + 2) + (INPUT + 3);

    // build a simple graph root with no caching
    let entry = Entry::new(|v: u32| {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        v + COUNTER.fetch_add(1, Ordering::Relaxed)
    });

    // branch off the always task 3 times
    let branch1 = entry.branch(|v| v);
    let branch2 = entry.branch(|v| v);
    let branch3 = entry.branch(|v| v);

    // join all three branches
    let join1 = branch1.join(branch2, |v1, v2| v1 + v2).cached();
    let join2 = join1.join(branch3, |v1, v2| v1 + v2);

    // execute the graph and test that the output is correct
    let output = join2.execute(INPUT);
    assert_eq!(output, OUTPUT);
}

#[tokio::test]
async fn future_graph() {
    // create test inputs and calculate expected output
    const INPUT: u32 = 100;
    const ADD_AMOUNT: u32 = 20;
    const SUB_AMOUNT: u32 = 10;
    const OUTPUT: u32 = (INPUT + ADD_AMOUNT + INPUT - SUB_AMOUNT) / 2;

    // create initial entrypoint that returns a future
    // this is shared and cached so it is only called once
    let entry = Entry::new(|v: u32| async move { v }).shared().cached();

    // create two branches off of the entrypoint that adds and subtracts
    let branch1 = entry.branch(|v| async move { v.await + 20 });
    let branch2 = entry.branch(|v| async move { v.await - 10 });

    // join both branches awaiting on both branches values
    let join = branch1.join(branch2, |v1, v2| async move { (v1.await + v2.await) / 2 });

    // execute and await the output, then check if the result is valid
    let output = join.execute(INPUT).await;
    assert_eq!(output, OUTPUT);
}

#[test]
fn weird_graph() {
    let i2 = Entry::new(|v: u32| v);
    let i3 = Entry::new(|v: u32| v);
    let i4 = Entry::new(|v: u32| v);

    let a2 = i2.branch(|v: u32| v * 2);
    let a3 = i3.branch(|v: u32| v + 6);

    let a1 = a2.join(a3, |v1, v2| println!("({v1}, {v2})"));
    let a4 = a3.join(i4, |v1, v2| println!("({v1}, {v2})"));

    let out1 = a1.execute(10);
    let out2 = a4.execute(20);
}

// #[graph::builder]
// pub async fn fetch_pricing(v1: u32, v2: u32, v3: u32) -> u32 {
//     v1.await + v2.await + v3.await
// }
