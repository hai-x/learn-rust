// enum List {
//     Cons(i32, List),
//     Nill,
// }

// fn main() {
//     println!("Hello, world!");
//     let mut z = 10;
//     let w = &mut z; // 可变引用
//     *w += 1;
// }

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let cons = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     println!("{:?},", cons);
// }

// use std::ops::Deref;

// #[derive(Debug)]
// struct MyBox<T> {
//     data: T,
// }
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox { data: x }
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.data
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);
//     // println!("{:?}", y);
//     assert_eq!(5, x);
//     println!("{:?}", y.data);
//     assert_eq!(5, *y);
// }

// #[derive(Debug)]
// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// fn main() {
//     let mut c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };
//     let d = CustomSmartPointer {
//         data: String::from("other stuff"),
//     };

//     // c.drop();
//     drop(c);
//     println!("CustomSmartPointers created.");
//     println!("{:?}", c);
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use std::rc::Rc;

// use crate::List::{Cons, Nil};

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating a = {}", Rc::strong_count(&a));

//     let c = Cons(4, Rc::clone(&a));
//     println!("count after creating a = {}", Rc::strong_count(&a));
// }

// use std::{cell::RefCell, rc::Rc};

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     let value = Rc::new(RefCell::new(5));
//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//     let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
//     *value.borrow_mut() += 10;

//     println!("a after = {a:?}");
//     println!("b after = {b:?}");
//     println!("c after = {c:?}");
// }

// use std::{cell::RefCell, rc::Rc};
// use List::{Cons, Nil};

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b)
//     }
//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));
// }

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
    vec,
};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         children: RefCell::new(vec![]),
//         parent: RefCell::new(Weak::new()),
//     });
//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//     let branch = Rc::new(Node {
//         value: 5,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });

//     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
// }

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
