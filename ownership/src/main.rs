fn main() {
    // {
    //     let s = "hello,world";
    // }
    // println!("{}", s);

    {
        let mut s = String::from("hello,world");
        s.push_str(", world!"); // push_str() 在字符串后追加字面值
        println!("{}", s);
    }
}
