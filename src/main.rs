extern crate rand;
//extern crate markov;

//use markov::Chain;

use std::io;
use std::io::Write;
mod maze;
mod traits;
mod items;
mod containers;
mod player;
mod utils;

fn main() {
    /*let mut chain = Chain::for_strings();
    chain.feed_str("white black red");//white or brown or black or yellow");
    println!("{}", chain.generate_str());*/

/*    let mut chains = vec![Chain::for_strings(), Chain::for_strings(), Chain::for_strings(), Chain::for_strings(), Chain::for_strings()];
    chains[0].feed_str("The white The black The brown The yellow The orange");
    chains[1].feed_str("dog cat rat mouse bat");
    chains[2].feed_str("jumped over hopped over dance over skipped over tipped over");
    chains[3].feed_str("the tall the small the round the big the huge the monstrous the proposterous");
    chains[4].feed_str("log. stump. root. step. stair. chair. bear.");
    println!("{}", chains.iter().fold(String::new(), |str, word| str + &word.generate_str()));*/
    //println!("{}", chain.generate_str() + &chain.generate_str());
    
    
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
