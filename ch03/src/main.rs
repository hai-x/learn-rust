// use std::io;

use std::io::{self};

fn main() {
    // ------ ch3.1 ------
    // let _i: isize = "-42".parse().expect("not a number");
    // let _j: u32 = 300000;
    // let _j: &str = "-3";
    // let mut _z = 3000;
    // _z = 31;

    // ------ ch3.2 ------
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of y is {y}");
    // let x1 = tup.0;
    // println!("The value of x is {x1}");

    // let a = [1, 2, 3, 4, 5];
    // println!("Plz enter an array index");
    // let mut idx: String = String::new();
    // io::stdin()
    //     .read_line(&mut idx)
    //     .expect("Failed to read line");
    // let idx: usize = idx.trim().parse().expect("Idx entered was not a number");
    // let ele = a[idx];
    // print!("The value of the element at index {idx} is {ele}")

    // ------ ch3.3 ------
    // fn five() -> isize {
    //     5
    // }

    // let x = five();
    // print!("The value of x is: {x}!")

    // ------ ch3.5 ------
    // let num = 3;
    // if num != 0 {
    //     //^^^ expected `bool`, found integer
    //     println!("error")
    // }
    // let condition: bool = true;
    // let num = if condition { 3 } else { 5 };
    // println!("num is {num}");

    // let mut count = 0;
    // let result = loop {
    //     count += 1;
    //     if count == 10 {
    //         break count + 3;
    //     }
    // };
    // println!("{result}")

    // let mut count = 0;
    // 'tag1: loop {
    //     let mut innerCount = 0;
    //     loop {
    //         println!("innerCount: {innerCount}");
    //         if innerCount == 5 {
    //             break 'tag1;
    //         }
    //         if innerCount == 10 {
    //             break;
    //         }
    //         innerCount += 1;
    //     }
    //     count += 1;
    // }
    // println!("count: {count}")
    // let mut lo: [i32; 5] = [1, 2, 3, 4, 5];

    // for i in (1..4).rev() {
    //     println!("i: {i}")
    // }

    // 1. 相互转换摄氏与华氏温度 c = 5 * (f-32)/9
    // fn c_to_f(c: f64) -> f64 {
    //     c * 9.0 / 5.0 + 32.0
    // }
    // let mut str = String::new();
    // io::stdin()
    //     .read_line(&mut str)
    //     .expect("Failed to read line");
    // let c: f64 = str.trim().parse().expect("not a 摄氏度");
    // let f = c_to_f(c);
    // println!("{f}°F {c}°C")

    // 2. 生成第 n 个斐波那契数
    // fn fab(n: usize) -> usize {
    //     if n == 1 {
    //         return 1;
    //     } else if n == 2 {
    //         return 2;
    //     }
    //     return fab(n - 1) + fab(n - 2);
    // }
    // let mut str = String::new();
    // io::stdin().read_line(&mut str).expect("fail to read");
    // let num = str.trim().parse().expect("not a number");
    // let fab_num = fab(num);
    // println!("fab of {num}: {fab_num}")

    // 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    let song = || {
        for day in 1..12 {
            println!(
                "On the {} day of Christmas my true love sent to me:",
                days[day]
            );
        }
    };

    song()
}
