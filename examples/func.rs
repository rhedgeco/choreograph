use choreo::{
    nodes::{Action, SyncExt, ThenExt},
    Node,
};

fn func_node<'a>(
    in1: impl Node<Output = u32>,
    in2: impl Node<Output = u32>,
) -> impl Node<Output = u32> {
    fn add_values(in1: u32, in2: u32) -> u32 {
        in1 + in2
    }

    Action::new(|| add_values(in1.exec(), in2.exec()))
}

fn main() {
    let source1 = Action::new(|| 10u32);
    let source2 = Action::new(|| 15u32);
    let source3 = Action::new(|| 19u32).synced();

    let func1 = func_node(
        source1,
        source3.clone().then(|v| {
            println!("into func1");
            v
        }),
    );
    let func2 = func_node(
        source2,
        source3.clone().then(|v| {
            println!("into func2");
            v
        }),
    );
    let func3 = func_node(
        func1,
        source3.clone().then(|v| {
            println!("into func3");
            v
        }),
    );
    let func4 = func_node(func2, func3);

    let out = func4.exec();
    println!("{out}");
}
