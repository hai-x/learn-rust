fn main() {
    // test1();
    // test2()
    // test3()
    test4()
}

// 引用与借用
// fn test1() {
//     fn get_len(s: &String) -> usize {
//         s.len()
//     }
//     fn fail_get_len(s: &String) {
//         // following is fail cause it cannot be borrowed as mutable
//         // s.push_str("try to push something to s !")
//     }
//     let s1 = String::from("hello,world");
//     println!("String->{} length: {}", s1, get_len(&s1));
// }

// 可变引用 mutable reference
// fn test2() {
//     fn change_string(s: &mut String) {
//         s.push_str(" have been changed")
//     }
//     let mut s = String::from("hello,world");
//     change_string(&mut s);
//     println!("{}", s)
// }

// 数据竞争（data race）
// 原因：
// 1.两个或更多指针同时访问同一数据。
// 2.至少有一个指针被用来写入数据。
// 3.没有同步数据访问的机制。
// fn test3() {
//     let mut s = String::from("hello,world");

//     // following can't do
//     // let s1 = &mut s;
//     // let s2 = &s; // cannot borrow `s` as immutable because it is also borrowed as mutable
//     // println!("{} {}", s1, s2)

//     // following can do
//     {
//         let s1 = &mut s;
//         s1.push_str(" have been changed");
//         println!("{}", s1)
//     }
//     let s2 = &s;
//     println!("{} ", s2) // hello,world have been changed
// }

// 悬垂引用（Dangling References）
// 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个悬垂指针
// 悬垂指针是其指向的内存可能已经被分配给其它持有者
fn test4() {
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    fn dangling() -> &String {
        let s = String::from("hello,world");
        &s
    }
    let dangling_ref = dangling();
}
