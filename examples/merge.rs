use choreo::{
    nodes::{merge, Source, ThenExt},
    GraphNode,
};

fn main() {
    let source1 = Source::new(|| 10);
    let source2 = Source::new(|| 25);
    let source3 = Source::new(|| 13);
    let source4 = Source::new(|| 8);
    let source5 = Source::new(|| 67);
    let merge = merge!(source1, source2, source3, source4, source5)
        .then(|(s1, s2, s3, s4, s5)| s1 + s2 + s3 + s4 + s5);

    println!("{}", merge.execute());
}
