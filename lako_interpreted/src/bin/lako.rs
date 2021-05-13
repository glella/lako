extern crate lako_interpreted; // load our library

use crate::lako_interpreted::frontend::scanner::A;

fn main() {
    println!("Hello from main");
    let first = A { a: 42 };
    println!("using scanner first: {:?}", first);
}
