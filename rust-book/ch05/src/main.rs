// struct User {
//     username: String,
// }

// struct Empty;

// struct Tuple1(i32, i32, i32);
// struct Tuple2(i32, i32, i32);
// fn main() {
//     let mut user: User = build_user();
//     println!("user.name: {:?}", user.username);
//     user.username = String::from("has been overwrite");
//     println!("user.name: {:?}", user.username);

//     let user2: User = User {
//         username: "new name".to_string(),
//         ..user
//     };

//     println!("user2.name: {:?}", user2.username);
//     println!("user.name: {:?}", user.username);

//     let _empty = Empty;

//     let _tuple1 = Tuple1(1, 2, 3);
//     let _tuple2 = Tuple1(2, 3, 4);

//     println!("tuple1: {:?} {:?} {:?}", _tuple1.0, _tuple1.1, _tuple1.2)
// }

// fn build_user() -> User {
//     let mut _user = User {
//         username: String::from("value"),
//     };
//     _user
// }

// fn shorthand_build_user(username: String) -> User {
//     User { username }
// }

// fn main() {
//     let rect1 = (30, 50);
//     println!("Area of the rect is {}", calc_area(rect1))
// }

// fn calc_area(rect: (i32, i32)) -> i32 {
//     rect.0 * rect.1
// }

// #[derive(Debug)]
// struct Rect {
//     width: i32,
//     height: i32,
// }

// fn main() {
//     let rect = Rect {
//         width: 30,
//         height: 50,
//     };
//     println!("Rect is {:#?}", rect);
//     println!("Area of the rect is {}", calc_area(&rect))
// }

// fn calc_area(rect: &Rect) -> i32 {
//     &rect.width * &rect.height
// }

// #[derive(Debug)]
// struct Rect {
//     width: i32,
//     height: i32,
// }

// impl Rect {
//     fn calc_area(&self) -> i32 {
//         self.width * self.height
//     }
// }
// fn main() {
//     let mut rect = Rect {
//         width: 30,
//         height: 50,
//     };
//     println!("{:?}", rect.calc_area());
//     rect.width = 40;
//     println!("{:?}", rect.calc_area());
// }

#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: 40,
        }
    }
}

fn main() {
    let rect = Rect::square(32);

    println!("rect is {:#?}", rect)
}
