use choreo::{
    nodes::{join, Source, ThenExt},
    GraphNode,
};

fn main() {
    let s1 = Source::new(10);
    let s2 = Source::new(25);
    let s3 = Source::new(13);
    let s4 = Source::new(8);
    let s5 = Source::new(67);
    let join = join!(s1, s2, s3, s4, s5);
    let add = join.then(|(s1, s2, s3, s4, s5)| s1 + s2 + s3 + s4 + s5);
    println!("{}", add.execute());
}
