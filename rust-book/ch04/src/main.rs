use String;
// use ferris_says::say;
// use std::io::{stdout, BufWriter};

// fn main() {
//     let s = String::from("hello world");
//     // let word_len = first_word_len(&s);
//     // s.clear();
//     // if use `slice`, it will cause Error because `clear` will borrow 's' but it has been borrow as immutable.
//     // println!("{} {}", word_len, s.len()) // 5 0
//     let word = first_word(&s);
//     println!("{}", word);

//     let std = stdout();
//     let msg = String::from("MSG: HELLO WORLD");
//     let len = msg.len();
//     let mut writer = BufWriter::new(std.lock());
//     say(&msg, len, writer).unwrap()
// }

// // fn first_word_len(s: &String) -> usize {
// //     let bytes = s.as_bytes();

// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             return i;
// //         }
// //     }
// //     s.len()
// // }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn main() {
//     {
//         // let mut s1 = String::from("hello world");
//         // let s2 = s1.clone();
//         // s1.push_str("im from rust");
//         // println!("s1: {s1}; s2: {s2}")

//         // let s1 = "1";
//         // let s2 = "2";
//         // let tuple = [s1, s2];
//         // let tuple2 = tuple;
//         // let tuple3 = ([tuple, ["3", "4"]]).concat();
//         // println!(
//         //     "tuple: {:?}\ntuple2: {:?}\ntuple3: {:?}",
//         //     tuple, tuple2, tuple3
//         // )

//         let str = String::from("value");
//         takes_os(str.clone());
//         let (mut str2, len) = return_os(str);
//         println!("str2: {:?}\nstr2_before_len:{len}", str2);

//         let str2_len = borrow_os(&str2);
//         println!("str2_after_len: {str2_len}");

//         {
//             borrow_os_and_update(&mut str2);
//         }
//         println!("str2: {str2}");
//     }
// }

use std::string;

fn takes_os(str: String) {
    println!("{str}")
}

fn return_os(mut str: String) -> (String, usize) {
    let len: usize = str.len();
    str.push_str("\n\tadd by return_os");
    (str, len)
}

fn borrow_os(s: &String) -> usize {
    return s.len() + 3;
}

fn borrow_os_and_update(s: &mut String) -> usize {
    s.push_str("\n\t\tand add byborrow_os_and_update");
    s.len()
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn for_in_3() {
    for i in 1..4 {
        println!("i: {:?}", { i })
    }
}

fn first_word_idx(s: &String) {
    let bytes = s.as_bytes();
    let byte_enum: std::iter::Enumerate<std::slice::Iter<u8>> = bytes.iter().enumerate();
    for (idx, &item) in byte_enum {
        println!("iter idx: {idx}, item: {item}");
        println!("{:?}", item == b' ')
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    &s[0..1]
}

fn main() {
    // for_in_3();

    // first_word_idx(&String::from("M E"));
    // let s: String = dangle();
    // println!("the slice of s: {:?}", &s[0..3])

    let mut s: String = String::from("value");

    let first = first_word(&s);
    s.clone().clear();
    println!("first: {first}");

    let arr = [1, 2, 3, 4, 5, 6, 7];
    assert_eq!(arr[1..3], [2, 3]);
    println!("slice of arr: {:?}", &arr[1..3]);
}

// 总结:
// - 引用: 指向堆数据的指针
// - 借用: 将 指向堆数据的指针 传递给另外一个作用域
//     - 可变引用: 可通过指针修改堆上数据
//     - 不可变引用: 只可通过指针读取堆上数据
//     - 规则: 存在不可变引用时，无法再获取可变引用，为了避免数据竞争问题
//       error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
