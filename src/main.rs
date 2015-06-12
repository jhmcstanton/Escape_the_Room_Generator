extern crate rand;

use std::io;
use std::io::Write;
mod maze;
mod traits;
mod items;
mod containers;
mod player;
mod utils;

fn main() {
    println!("What is your name?");
    print!(">>> "); io::stdout().flush();

    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .ok()
        .expect("Failed to readline, looks like you can't play :(");
    
    let mut maze = maze::Maze::new(100, &name);
    println!("Hello, world!");

    maze.help();

    loop {
        if maze.take_input() {
            break;
        }
    }
}
