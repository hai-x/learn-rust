// use std::slice;

// fn main() {
//     println!("Hello, world!");

//     let mut num = 5;
//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;

//     // println!("r1: {}, r2: {}", r1, r2);

//     let address = 0x012345usize;
//     let r = address as *const i32;

//     unsafe {
//         *r2 = 300;
//         println!("r1 is: {}", *r1);
//         println!("r2 is: {}", *r2);
//     }

//     fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//         let len = values.len();
//         let ptr = values.as_mut_ptr();
//         assert!(mid <= len);

//         // (&mut values[..mid], &mut values[mid..])
//         unsafe {
//             (
//                 slice::from_raw_parts_mut(ptr, mid),
//                 slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//             )
//         }
//     }

//     let mut v = vec![1, 2, 3, 4, 5, 6];

//     let r = &mut v[..];

//     let (a, b) = split_at_mut(r, 3);

//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);

//     let address = 0x01234usize;
//     let r = address as *mut i32;
//     let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000000) };
//     println!("values: {:?}", values);

// }

// use std::ops::Add;

// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;
//     fn add(self, rhs: Self) -> Self::Output {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(Point { x: 1, y: 1 } + Point { x: 2, y: 2 }, {
//         Point { x: 3, y: 3 }
//     })
// }

// use std::fmt;

// trait OutlinePrint: fmt::Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {output} *");
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// enum E {
//     None,
//     Some(i32, i32),
// }
// impl OutlinePrint for E {}

// fn main() {
//     let p = Point { x: 1, y: 2 };
//     p.outline_print();
// }

// use std::fmt;
// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// fn main() {
//     let w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
//     println!("w: {}", w);
// }

// type Result<T> = std::result::Result<T, std::io::Error>;

// fn main() {
//     fn t() -> () {
//         println!("hello");
//     }
//     t();

//     // let s2: str = "How's it going?";
// }

// fn main() {
//     fn add_one(x: i32) -> i32 {
//         x + 1
//     }
//     fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//         f(arg) + f(arg)
//     }

//     let answer = do_twice(add_one, 5);
//     println!("Answer = {}", answer);
// }

// #[derive(Debug)]
// enum Status {
//     Value(u32),
//     Stop,
// }
// fn return_closure() -> Box<dyn Fn(i32) -> i32> {
//     Box::new(|x| x + 1)
// }

// fn main() {
//     let list_of_statuses: Vec<Status> = (0u32..10).map(Status::Value).collect();
//     println!("{:?}", list_of_statuses);

//     let closure = return_closure();
//     let v = closure(3);
//     println!("{}", v)
// }

// 宏
// 自定义宏 #[derive] 结构体 + 枚举
// 类属性宏 任意项
// 类函数宏 像函数 作为参数传递的 token

#[macro_export]
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec= Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Hello {}

fn main() {}
