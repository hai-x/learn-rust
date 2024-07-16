#[cfg(a = "unix")]
pub fn unix() {
    println!("run in unix!")
}

fn main() {
    let match_kind = if let cfg!(unix) = true {
        "unix"
    } else {
        "unknown"
    };

    println!("run in {}!!!!", match_kind)
}
