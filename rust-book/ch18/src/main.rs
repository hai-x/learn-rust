fn main() {
    // 所有可能会用到模式的位置

    // match
    let x = Some(1);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    // if let
    let s = Some(15);
    if let Some(x) = s {
        println!("{}", x);
    };

    // while let
    let mut stack = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    for (idx, v) in v.iter().enumerate() {
        println!("{} {}", idx, v)
    }

    // let
    let (x, y) = (1, 2);

    // 函数参数
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("({}, {})", x, y);
    }
    let p = (3, 4);
    print_coordinates(&p);

    fn print_coordinates_2(x: &[i32]) {
        println!("{x:?}");
    }
    print_coordinates_2(&[3]);

    let some_option_value = Some(5);
    if let None = some_option_value {
        println!("{x}");
    } else {
        println!("None");
    }

    let x = 1;
    match x {
        1 => println!("{}", x),
        _ => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("found 50"),
        Some(y) => println!("found {}", y),
        _ => println!("Default case, x = {x:?} = {y:?}"),
    }

    let x = 1;
    match x {
        1 | 2 => println!("{}", x),
        _ => (),
    }

    let x = 5;
    match x {
        1..=5 => println!("{}", x),
        _ => (),
    }

    let x = 'c';
    match x {
        'a'..='z' => println!("{}", x),
        _ => (),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("a: {}, b: {}", a, b);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg2 = Message2::ChangeColor(Color::Rgb(0, 160, 255));
    match msg2 {
        Message2::ChangeColor(Color::Rgb(a, b, c)) => {
            println!("Change the color to red {a}, green {b}, and blue {c}")
        }
        _ => (),
    }

    struct A {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = A { x: 0, y: 0, z: 0 };
    match origin {
        A { x, .. } => println!("x:{x}"),
    }

    let t = (1, 2, 3, 4);
    let (.., z) = t;
    println!("z:{}", z);

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("{}", x),
        _ => (),
    }
}
