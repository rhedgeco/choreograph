use std::{fmt::Display, ops::Add};

use choreograph::{Node, node::Task};

pub fn custom_node<T: Display + Add<usize, Output = usize>>(
    input0: impl Node<Output = usize>,
    input1: impl Node<Output = T>,
) -> impl Node<Output = usize> {
    Task::new(|| {
        let input0 = input0.execute();
        let input1 = input1.execute();
        println!("input1 before: {}", input1);
        println!("input1 + 10: {}", input1 + 10);
        input0 + 20
    })
}

fn main() {
    let input0 = Task::wrap(123);
    let input1 = Task::wrap(456);
    let node = custom_node(input0, input1);
    let output = node.execute();
    println!("final output: {output}");
}
