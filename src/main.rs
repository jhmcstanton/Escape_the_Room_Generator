extern crate rand;

use std::io;

mod maze;
mod traits;
mod items;
mod containers;
mod player;
mod utils;

fn main() {
    println!("What is your name?");

    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .ok()
        .expect("Failed to readline, looks like you can't play :(");
    
    let maze = maze::Maze::new(100000, &name);
    println!("Hello, world!");
}
