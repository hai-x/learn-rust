fn main() {
    // 1.字面量 离开作用域 底层自动触发drop函数进行GC
    // {
    //     let s = "hello,world";
    // }
    // println!("{}", s);

    // 2.创建字符串引用
    // {
    //     let mut s = String::from("hello,world");
    //     s.push_str(", world!"); // push_str() 在字符串后追加字面值
    //     println!("{}", s);
    // }

    // 3.引用地址转移 无法再次使用原引用 ❎浅拷贝
    // {
    //     let s1 = String::from("hello");
    //     let s2 = s1;
    //     println!("{} world!", s1)
    // }

    // 4.使用克隆进行复制堆上数据
    // {
    //     let s1 = String::from("hello");
    //     let mut s2 = s1.clone();
    //     s2.push_str(" world!");
    //     println!("s1:{}, s2:{}", s1, s2)
    // }

    // 5.使用拷贝进行复制栈上数据
    // {
    //     let x = 5;
    //     let y = x;
    //     println!("{},{}", x, y)
    // }

    // 6.使用Copy trait特殊注解，那么一个旧变量在将其自身赋值给其他变量后仍可用。
    // 不允许实现了Drop trait注解后再实现Copy trait，会编译错误
    // 因为不允许离开作用域后再去使用该变量（已被Drop回收）
    // 以下类型实现了Copy trait
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

    test7();
    test8();
    test9();
}

// 7.所有权与函数
// 将值传递给函数可能会移动或者复制，就像赋值语句一样。
fn test7() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // s的内存被释放，编译错误

    let x = 5;
    makes_copy(x);
    println!("main {}", x); // x被拷贝后进入makes_copy函数，其内存未被释放，仍可使用。
}
fn takes_ownership(some_string: String) {
    println!("takes_ownership fn : {}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("makes_copy {}", some_int)
}

// 8.返回值与作用域
// 返回值也可以转移所有权。
fn test8() {
    let s1 = gives_ownership();
    let s2 = String::from("s2");
    let s3 = takes_and_gives_back(s2.clone()); // s2如果不用clone，编译会报错，其内存会转移给s3

    println!("s1:{},s2:{},s3:{}", s1, s2, s3)
}

fn gives_ownership() -> String {
    let s = String::from("s1");
    s
}
fn takes_and_gives_back(s: String) -> String {
    s
}

// 9.可以使用元祖来返回多个值，
fn test9() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_len(s1);
    println!("the length of {} is {}.", s2, len)
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
