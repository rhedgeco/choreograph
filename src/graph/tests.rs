use crate::graph::{node::NodeExt, BranchExt, GraphRoot, JoinExt};

#[test]
fn simple_branch_join() {
    // create test inputs and calculate expected output
    const TEST_INPUT: u32 = 50;
    const INITIAL_ADD: u32 = 100;
    const ADD_AMOUNT: u32 = 20;
    const SUB_AMOUNT: u32 = 10;
    const TEST_OUTPUT: u32 =
        ((TEST_INPUT + INITIAL_ADD + ADD_AMOUNT) + (TEST_INPUT + INITIAL_ADD - SUB_AMOUNT)) / 2;

    // build the tasks that branch off from root then join at end
    let root_task = GraphRoot::new(|v: u32| v + 100);
    let add_task = root_task.branch(|v| v + 20);
    let sub_task = root_task.branch(|v| v - 10);
    let join_task = add_task.join(sub_task, |(v1, v2)| (v1 + v2) / 2);

    // execute the task and check if it is valid
    let output = join_task.execute(TEST_INPUT);
    assert_eq!(output, TEST_OUTPUT);
}
