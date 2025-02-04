use std::time::{Duration, Instant};

use choreo::{
    nodes::{Action, SyncExt, ThenExt},
    Node,
};

// create an action that takes a long time
fn slow_action(input: u32) -> u32 {
    std::thread::sleep(Duration::from_secs(2));
    input * 10
}

fn main() {
    // create a source value
    let source = Action::new(|| 6)
        .then(|v| {
            // use a then statement to print the starting value
            println!("using starting value {v}");
            v
        })
        // sync the node so it can be forked
        .synced();

    // clone the source 3 times
    let fork1 = source.clone();
    let fork2 = source.clone();
    let fork3 = source.clone();

    // create 3 slow actions
    let slow1 = Action::new(|| slow_action(fork1.execute() + 7));
    let slow2 = Action::new(|| slow_action(fork2.execute() + 8));
    let slow3 = Action::new(|| slow_action(fork3.execute() + 9));

    // merge the slow nodes in seperate threads
    let merge = Action::new(|| {
        let v1 = std::thread::spawn(|| slow1.execute());
        let v2 = std::thread::spawn(|| slow2.execute());
        let v3 = std::thread::spawn(|| slow3.execute());
        v1.join().unwrap() + v2.join().unwrap() + v3.join().unwrap()
    });

    // measure the execution time
    let instant = Instant::now();
    let output = merge.execute();
    let duration = instant.elapsed().as_secs_f64();

    // print the result
    println!("Output (expected 420): {output}");
    println!("Duration (expected ~2 seconds): {duration} seconds");
}
