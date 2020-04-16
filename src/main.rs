use derive::Test;

use crate::traits::Test;

mod traits;

#[derive(Test)]
struct Cool {}

fn main() {
    println!("Hello, world!");

    let cool = Cool {};

    println!("Test: {}", cool.test_fn());
}
