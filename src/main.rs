extern crate rand;

mod generator;
mod traits;
mod items;
mod containers;

fn main() {
    let maze = generator::Maze::new(100000);
    println!("Hello, world!");
}
