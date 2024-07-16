// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {result}");

//     let res = largest(&char_list);
//     println!("The largest char is {res}");
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

// mod aggregator;

// use std::fmt::Debug;

// use aggregator::{NewArticle, Summary};

// fn main() {
//     let article: NewArticle = NewArticle {
//         headline: String::from("headline"),
//         location: String::from("location"),
//         author: String::from("author"),
//         content: String::from("content"),
//     };

//     println!("1 new article: {}", article.summarize());

//     notify(&article);
// }

// fn notify(item: &(impl Summary + Debug)) {
//     println!("by notify: {}", item.summarize());
// }

// mod pair;
// use std::fmt::Display;

// use pair::{Pair, A};

// fn main() {
//     let _pair = Pair::new(1, 2);
//     _pair.test();
// }

// fn main() {
//     let str1 = String::from("abcd");
//     let res;
//     {
//         let str2 = "xyz";
//         // let str2 = String::from("xyz");
//         res = longest(&str1, &str2);
//     }
//     println!("{res:?}");
// }

// 'a 取最短的声明周期的引用
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let str1 = String::from("abcd");
//     let res;
//     {
//         // let str2 = "xyz";
//         let str2 = String::from("xyz");
//         res = longest(&str1, &str2);
//     }
//     println!("{res:?}");
// }

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     // if x.len() > y.len() {
//     //     x
//     // } else {
//     //     y
//     // }
//     x
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("call me xxx, some years ago...");
//     let first_sentence;
//     {
//         first_sentence = novel.split(".").next().unwrap();
//     }
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = "abc".to_string();
    let y = "defg".to_string();
    println!(
        "{:?}",
        longest_with_an_announcement(&x, &y, "im announcement!")
    )
}
