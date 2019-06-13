extern crate liballistic;

use liballistic::conversions::c_to_f;

fn main() {
    println!("Test.rs: main");
    println!("25°C = {}°F", c_to_f(25.0));
}
