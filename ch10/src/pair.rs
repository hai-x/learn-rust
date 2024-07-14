use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

pub trait A {
    fn test(&self) {
        println!("test");
    }
}
impl<T: Display> A for Pair<T> {
    fn test(&self) {
        println!("Pair test, x:{} y:{}", self.x, self.y);
    }
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("larger is x = {}", self.x)
        } else {
            println!("larger is y = {}", self.y)
        }
    }
}
