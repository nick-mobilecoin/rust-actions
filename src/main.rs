use std::time::SystemTime;


fn main() {
    let start = SystemTime::now(); let delta = start.elapsed();
    println!("The delta is {delta:?}");

    println!("Hello, world!");
}
