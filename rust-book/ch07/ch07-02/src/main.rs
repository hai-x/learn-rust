// 别名
use crate::garden::veg;

// 声明子模块
// 模块里的代码默认对其父模块私有
mod garden;

use crate::test::test1;

fn main() {
    let plant: veg::Asparagus = veg::Asparagus {
        name: String::from("Asparagus"),
    };
    println!("{plant:?}");
    test1::log();
}

mod test {
    pub mod test1 {
        use super::super::garden;
        enum _Enum {
            Act,
        }
        pub fn log() {
            println!(
                "garden: {:?}",
                garden::veg::Asparagus {
                    name: String::from("val")
                }
            )
        }
    }
}
