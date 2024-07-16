// use guess::Guess;

// fn main() {
//     println!("Hello, world!");
//     // panic!("err");

//     let v = vec![1, 2, 3];
//     v[99];
// }

pub mod guess;
// mod guess;

mod guess1;

use guess::A;

use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

// fn main() {
//     let file_res = File::open("h.txt");
//     let file = match file_res {
//         Ok(file) => file,
//         Err(err) => match err.kind() {
//             ErrorKind::NotFound => match File::create("h.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("create error"),
//             },
//             other => {
//                 panic!("Other error: {other:?}")
//             }
//         },
//     };
//     println!("{file:?}")
// }

fn main() {
    let x = 10;
    // let add_x = |/* Type */ y| x + y;

    // let file = File::open("1h.txt").expect("h.txt should be existed");

    // println!("{}", read_user_from_file().expect("error"));

    println!("guess: {:?}", guess_fn());
}

// fn read_user_from_file() -> Result<String, io::Error> {
//     let username_file_res = File::open("h.txt");
//     let mut username_file = match username_file_res {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => return Err(e),
//     }
// }

// #[derive(Debug)]
// enum OurError {
//     IoError(io::Error),
// }

// impl From<io::Error> for OurError {
//     fn from(value: io::Error) -> OurError {
//         OurError::IoError(value)
//     }
// }

// fn read_user_from_file() -> Result<u32, OurError> {
//     let username_file_res = File::open("h1.txt")?;
//     Ok(1)
// }

// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }

fn guess_fn() -> A {
    A::new(1)
}
