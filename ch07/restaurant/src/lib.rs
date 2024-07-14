pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::io::{self, Write};

pub mod son;

use son::Son as renamed_son;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let name = renamed_son::name(String::from("foo"));
        println!("{:?}", name);
        // assert!(name, "foo");
        front_of_house::hosting::add_to_waitlist()
    }
}

mod front_of_house {
    pub mod hosting {

        pub fn add_to_waitlist() {
            println!("add_to_waitlist")
        }
        fn seat_at_table() {
            println!("seat_at_table");
        }
    }
    mod serving {
        fn take_order() {
            println!("take_order")
        }
        fn serve_order() {
            println!("serve_order")
        }
        fn take_payment() {
            println!("take_payment")
        }
    }
}
