// 每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数。
// 用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。

// #[derive(Debug)]
// enum IpAddr {
//     V4 { width: isize, height: isize },
//     V6(),
//     V10(),
// }

// impl IpAddr {
//     fn test(self: &Self) {
//         println!("{:?}", self)
//     }
// }
// fn main() {
//     println!(
//         "{:?}",
//         IpAddr::V4 {
//             width: 3,
//             height: 4
//         }
//     );

//     IpAddr::test(&IpAddr::V10());

//     let mut absent_num: Option<i32> = None;
//     absent_num = Some(4);
// }
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alabama));
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Dime => 10,
//         Coin::Nickel => 5,
//         Coin::Penny => 1,
//         Coin::Quarter(state) => {
//             println!("State is {state:?}");

//             1
//         }
//     }
// }

// fn main() {
//     let pluz = plus_one(Some(4));
//     println!("{pluz:?}")
// }

// fn plus_one(x: Option<u8>) -> Option<u8> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let dice_roll = 9;
//     let c: u8 = match dice_roll {
//         3 => a(),
//         _ => c(),
//     };
//     println!("{c}")
// }

// fn a() -> u8 {
//     3
// }

// fn c() -> u8 {
//     255
// }

fn main() {
    let conf_max = Some(3u8);
    match conf_max {
        Some(max) => println!("max: {max}"),
        _ => (),
    }

    let mut count = 133;
    if let 132 = count {
        println!("{count:?}")
    } else {
        println!("null")
    }
}
