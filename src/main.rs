extern crate cacher;

use std::thread;
use std::time::Duration;

use cacher::Cacher;

fn main() {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    println!("Today, do {} pushups", expensive_result.value(10));
    println!("Next, do {} situps", expensive_result.value(8));
}
