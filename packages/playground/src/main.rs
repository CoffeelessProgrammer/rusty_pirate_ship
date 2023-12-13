#![allow(dead_code)]

mod shapes;
mod formatting;

fn main() {
    println!("Hello, playground!");
    formatting::run();
    shapes::run();
}