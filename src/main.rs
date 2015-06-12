extern crate rand;

mod maze;
mod traits;
mod items;
mod containers;
mod player;

fn main() {
    let maze = maze::Maze::new(100000);
    println!("Hello, world!");
}
