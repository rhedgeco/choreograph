fn main() {}

pub struct TestStruct {
    value: u32,
}

pub struct Wrap<'a>(&'a mut TestStruct);

#[choreo::graph]
impl TestStruct {
    pub fn create_node(&mut self, add_value: u32) -> Wrap {
        self.value += add_value;
        println!("im doing a cool thing!");
        Wrap(self)
    }
}
