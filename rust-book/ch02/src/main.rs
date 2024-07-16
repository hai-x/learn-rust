use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");

    let default = 2;
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(default), 9);

    // let x: Result<u32, &str> = Err("error");
    // assert_eq!(x.unwrap_or(default), default);

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail err");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => print!("Too small!"),
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }

    //   {
    //     Ok(r) => {
    //         println!("{r} bytes read");
    //         println!("success")
    //     }
    //     Err(e) => {
    //         println!("{e} err")
    //     }
    // };
}
