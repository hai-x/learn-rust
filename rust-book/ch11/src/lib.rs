pub mod adder {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, io::stdin};

    use super::*;

    #[test]
    fn it_works() {
        let result = adder::add(2, 2);
        assert!(result == 4, "right res");
    }
    #[test]
    fn another() -> Result<(), String> {
        // assert_eq!(&String::from("!1"), &String::from("!1"))
        // panic!("111")

        let mut inp = String::new();
        stdin().read_line(&mut inp).expect("can not read line");

        let num = inp.trim().parse::<u32>().unwrap();

        if num > 3 {
            Ok(())
        } else {
            Err(String::from("num <= 3"))
        }
    }
}
