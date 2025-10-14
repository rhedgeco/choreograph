use choreograph::{
    Node,
    node::{Task, ThenExt},
};

fn main() {
    // create two simple tasks
    let input1 = Task::wrap(6);
    let input2 = Task::wrap(9);

    let action = Task::new(|| {
        // execute both nodes inside another task
        input1.resolve() + input2.resolve()
    })
    // then multiply the ouput value by ten
    .then(|value| value * 10);

    // execute and print the final output
    println!("Output: {}", action.resolve());
}
