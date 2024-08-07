#[derive(Debug)]
pub struct A {
    value: i32,
}

impl A {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value:?}.");
        }
        A { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
