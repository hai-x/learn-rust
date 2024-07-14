// use std::{collections::linked_list, thread, time::Duration, vec};

// #[derive(Debug, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;
//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Blue => num_blue += 1,
//                 ShirtColor::Red => num_red += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     fn add_one_v1(x: u32) -> u32 {
//         x + 1
//     }
//     let add_one_v2 = |x: u32| -> u32 { x + 1 };
//     // let add_one_v3 = |x| x + 1;
//     // let add_one_v4 = |x| x + 1;

//     let expensive_closure = |num: u32| -> u32 {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
//     let s = expensive_closure(5);

//     println!("Hello, world!");

//     let store = Inventory {
//         shirts: vec![
//             ShirtColor::Blue,
//             ShirtColor::Blue,
//             ShirtColor::Blue,
//             ShirtColor::Blue,
//             ShirtColor::Red,
//             ShirtColor::Red,
//             ShirtColor::Red,
//             ShirtColor::Red,
//         ],
//     };
//     let user_pref1 = Some(ShirtColor::Blue);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2: Option<ShirtColor> = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
// }

// use std::{thread, vec};

// fn main() {
//     // let list = vec![1, 2, 3];
//     // println!("Before defining closure: {list:?}");

//     // let only_borrows = || println!("From closure: {list:?}");

//     // println!("Before calling closure: {list:?}");
//     // only_borrows();
//     // println!("After calling closure: {list:?}");

//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     let mut borrows_mut = || list.push(7);
//     borrows_mut();
//     println!("Before calling closure: {list:?}");
//     println!("after calling closure: {list:?}");
// }

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before calling closure: {list:?}");

//     thread::spawn(move || print!("Form thread: {list:?}"))
//         .join()
//         .unwrap();

// println!("after calling closure: {list:?}");
// }

// enum Opt<T> {
//     Some(T),
//     None,
// }

// impl<T> Opt<T> {
//     pub fn unwrap_or_else<F>(this: Option<T>, f: F) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         match this {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: i32,
//     height: i32,
// }

// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];

//     list.sort_by_key(|r| r.width);
//     println!("{list:#?}");
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];

//     let mut num_sort_operations = 0;
//     list.sort_by_key(|r: &Rectangle| {
//         num_sort_operations += 1;
//         r.width
//     });
//     println!("{list:#?}, sorted in {num_sort_operations} operations");
// }

// fn main() {
//     let v1 = vec![1, 2, 3, 3];
//     let mut v1_iter = v1.iter();
//     // for val in v1_iter {
//     //     print!("Got: {val}");
//     // }
//     // v1_iter.next();
//     // let total: i32 = v1_iter.sum();
//     // assert_eq!(total, 6);

//     v1_iter
//         .map(|x| {
//             println!("11");
//             x + 1
//         })
//         .collect::<Vec<_>>();
// }

// use std::vec;

// #[derive(Debug, PartialEq)]
// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     shoes.into_iter().filter(|s| s.size == shoe_size).collect()
// }

// fn main() {
//     let shoes = vec![
//         Shoe {
//             size: 10,
//             style: String::from("sneaker"),
//         },
//         Shoe {
//             size: 13,
//             style: String::from("sandal"),
//         },
//         Shoe {
//             size: 10,
//             style: String::from("boot"),
//         },
//     ];

//     let in_my_size = shoes_in_size(shoes, 10);

//     assert_eq!(
//         in_my_size,
//         vec![
//             Shoe {
//                 size: 11,
//                 style: String::from("sneaker"),
//             },
//             Shoe {
//                 size: 10,
//                 style: String::from("boot"),
//             },
//         ]
//     )
// }

fn main() {
    let mut buffer: Vec<i32> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];
    let coefficients: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2];
    let qlp_shift: i16 = 1;
    println!("{}", 16 >> qlp_shift);
    for i in 12..buffer.len() {
        println!("{i}");
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s)
            .sum::<i32>()
            >> qlp_shift;
    }
}
