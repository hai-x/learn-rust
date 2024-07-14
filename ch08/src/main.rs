// fn main() {
//     println!("Hello, world!");

//     let mut v: Vec<i32> = Vec::new();
//     v.push(1);
//     v.push(3);
//     v.push(10);

//     let v1 = vec![1, 2, 3];
//     // if Some(&v1[100]) == Option::None {
//     //     println!("none")
//     // } else {
//     //     println!("{:?}", &v1[100])
//     // }
//     match v1.get(100) {
//         None => {
//             println!("none")
//         }
//         Some(v) => {
//             println!("{v}")
//         }
//     }

//     let v2 = vec![1, 2, 3, 4, 5];
//     for i in &v2 {
//         println!("{i}")
//     }
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     let first = &v[0];

//     v.push(6);

//     println!("The first element is: {first}");
// }

// fn main() {
//     let s1 = "s1".to_string();
//     let s2 = "s2".to_string();
//     let s3 = s1 + "-" + &s2;
//     // println!("{s1}") // error
//     let s4 = format!("{s3}-{s2}");
//     println!("{s4}");

//     println!("{:?}", s4.as_bytes())
// }

use std::{collections::HashMap, io, iter::Map, num::Saturating};

// fn main() {
//     let mut score = HashMap::new();
//     let s1 = String::from("1");
//     let s2 = String::from("2");
//     let s3 = String::from("3");
//     score.insert(&s1, 1);
//     score.insert(&s2, 2);
//     score.insert(&s3, 3);
//     println!("{score:?}");
//     println!("{s1:?}")
// }

// fn main() {
//     let (median, mode) = calculate_median_and_mode(vec![11, 9, 23, 14, 5]);
//     println!("{:?}", format!("{median}, {mode}"));
// }

// // 1,2,3,4,5,6
// fn calculate_median_and_mode(v: Vec<i32>) -> (f32, i32) {
//     let mut _v = v.clone();
//     _v.sort();
//     println!("{_v:?}");
//     let len = _v.len();
//     let median = if len % 2 == 0 {
//         (_v[len / 2 - 1] + _v[len / 2]) as f32 / 2.0
//     } else {
//         _v[len / 2] as f32
//     };
//     let mut map = HashMap::new();
//     for &v in &_v {
//         map.entry(v).or_insert(0);
//     }

//     return (median, 3);
// }

// enum Department {
//     Engineering,
//     Sales,
// }

// struct Employee {
//     name: String,
//     age: u32,
// }

// type Company = HashMap<String, Vec<String>>;

// fn main() {
//     let mut company: Company = HashMap::new();
//     loop {
//         println!("Please enter a command (e.g., 'Add Sally to Engineering' or 'List all'):");
//         let mut inp = String::new();
//         io::stdin()
//             .read_line(&mut inp)
//             .expect("Failed to read line");
//         let input = inp.trim();
//         if input.starts_with("Add") {
//             add_employee(&mut company, input);
//         } else if input.starts_with("List") {
//             if input == "List all" {
//                 list_all_employees(&mut company)
//             }
//         }
//     }
// }

// fn add_employee(company: &mut Company, input: &str) {
//     let parts: Vec<&str> = input.split_whitespace().collect();
//     if parts.len() >= 4 && parts[0] == "Add" && parts[2] == "to" {
//         let name = parts[1].to_string();
//         let department = parts[3..].join(" ");
//         let _department = department.clone();
//         let employees = company.entry(department).or_insert_with(Vec::new);
//         employees.push(name);
//         println!("Added {} to {}", parts[1], _department);
//     } else {
//         println!("Invalid command format. Use 'Add <Name> to <Department>'");
//     }
//     // let engineering_vec: Vec<Employee> = vec![];
//     // let sales_vec: Vec<Employee> = vec![];
//     // map.insert(Department::Engineering, engineering_vec);
//     // map.insert(Department::Sales, sales_vec);
// }

// fn list_all_employees(company: &mut Company) {
//     let mut departments: Vec<&String> = company.keys().collect();
//     departments.sort();
//     for department in departments {
//         let mut employees = company.get(department).unwrap().clone();
//         employees.sort();
//         println!("Employees in {}:", department);
//         for employee in employees {
//             println!("{}", employee);
//         }
//     }
// }

fn main() {
    println!("{:?}", Module::default())
}

#[derive(Debug)]
struct Module {
    module_type: String,
}
impl Default for Module {
    fn default() -> Self {
        Module {
            module_type: String::from("Normal"),
        }
    }
}
