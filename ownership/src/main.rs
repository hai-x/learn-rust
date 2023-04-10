fn main() {
    let s = String::from("hello world");
    // let word_len = first_word_len(&s);
    // s.clear();
    // if use `slice`, it will cause Error because `clear` will borrow 's' but it has been borrow as immutable.
    // println!("{} {}", word_len, s.len()) // 5 0
    let word = first_word(&s);
    println!("{}", word)
}

// fn first_word_len(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
