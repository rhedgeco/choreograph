use choreograph::{Task, task::ThenExt};

fn main() {
    // create two simple tasks
    let input1 = || 6;
    let input2 = || 9;

    let action = (|| {
        // execute both tasks inside another task
        input1() + input2()
    })
    // tasks can be chained using `then` function
    .then(|value| value * 10);

    // execute and print the final output
    println!("Output: {}", action.execute());
}
