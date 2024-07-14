pub trait Draw {
    fn draw(&self) {}
}

pub struct Button {
    pub width: i32,
    pub length: i32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("width: {:?},length: {:?}", self.width, self.length);
    }
}

// pub struct Screen<T: Draw> {
//     pub comps: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.comps.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
