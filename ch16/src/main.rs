use std::{
    rc::Rc,
    sync::{
        mpsc::{self, channel},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

// fn main() {
//     println!("Hello, world!");

//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i}");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// fn main() {
//     println!("Hello, world!");

//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i}");
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//     handle.join().unwrap();
//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     let handle = thread::spawn(move || println!("vec: {v:?}"));
//     handle.join().unwrap()
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap()
//     });

//     thread::sleep(Duration::from_secs(1));
//     let received = rx.try_recv().unwrap();
//     println!("Got: {received}")
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         let s = String::from("val");
//         tx.send(s).unwrap();
//         println!("{s:?}");
//     });
//     let sth = rx.recv().unwrap();
//     println!("got: {sth:?}")
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//     for received in rx {
//         println!("Got: {received}");
//     }
// }

// fn main() {
//     // --snip--

//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {received}");
//     }

//     // --snip--
// }

// fn main() {
//     let m = Mutex::new(5);

//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//     println!("m= {m:?}")
// }

// fn main() {
//     let b = Box::new(Box::new(Box::new(Box::new("box"))));
//     println!("b: {:?}", *b);
// }

// fn main() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }
//     println!("Result: {}", *counter.lock().unwrap());
// }

// fn dead_lock() {
//     let r1 = Arc::new(Mutex::new(0));
//     let r2 = Arc::new(Mutex::new(1));

//     let res1 = Arc::clone(&r1);
//     let res2 = Arc::clone(&r2);

//     let thread1 = thread::spawn(move || {
//         let lock1 = res1.lock().unwrap();
//         println!("Thread 1: Locked resource 1");
//         thread::sleep(Duration::from_millis(50));
//         let lock2 = res2.try_lock();
//         if let Err(err) = lock2 {
//             // drop(lock2);

//             println!("thread1 panic");
//         }
//         // drop(lock1);
//         // drop(lock2);
//     });

//     let res1 = Arc::clone(&r1);
//     let res2 = Arc::clone(&r2);

//     let thread2 = thread::spawn(move || {
//         let lock2 = res2.lock().unwrap();
//         println!("Thread 2: Locked resource 2");
//         thread::sleep(Duration::from_millis(50));
//         // let lock1 = res1.try_lock();
//         // println!("Thread 2: Locked resource 1");

//         let lock1 = res1.try_lock();
//         if let Err(err) = lock1 {
//             println!("thread2 panic");
//         }
//     });
//     thread1.join().unwrap();
//     thread2.join().unwrap();
// }

// fn main() {
//     dead_lock();
// }

use std::fmt;

#[derive(Debug, Copy, Clone)]
struct MyType {
    value: i32,
}

impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct has value: {}", self.value)
    }
}

// unsafe impl Sync for MyType {}

fn main() {
    let my_type = MyType { value: 31 };
    let handles = (0..10).map(|_| {
        thread::spawn(move || {
            // let t = Arc::new(&);
            println!("value: {}", my_type)
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
}
