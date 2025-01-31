use choreo::{
    nodes::{join, Data, ThenExt},
    GraphNode,
};

fn main() {
    let s1 = Data::new(10);
    let s2 = Data::new(25);
    let s3 = Data::new(13);
    let s4 = Data::new(8);
    let s5 = Data::new(67);
    let join = join!(s1, s2, s3, s4, s5);
    let add = join.then(|(s1, s2, s3, s4, s5)| s1 + s2 + s3 + s4 + s5);
    println!("{}", add.execute());
}
