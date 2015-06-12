extern crate rand;
extern crate std; 

use rand::Rng;
use std::cmp;
use std::io;
use std::io::Write;
use std::convert::AsRef;

use items;
use containers;
use utils;
use traits::{Describable, Searchable, Breakable};
use player::Player;


pub struct Maze<'a: 'b, 'b> {
    start: InitialRoom,
    maze : MazePath<'a>,
    player: Player<'a, 'b>
}

pub struct Door {
    locked: bool
}

pub struct Exit {
    locked: bool
}

pub struct InitialRoom;

pub enum MazePath<'a> {
    Room {
        door: Door,
        containers: Vec<containers::Container<'a>>
    },
    Connector {
        door: Door,
        other_rooms: Vec<MazePath<'a>>,
        containers: Vec<containers::Container<'a>>
    },
    Exit {
        entrance: Door,
        exit: Exit,
        containers: Vec<containers::Container<'a>>
    }
}

impl Exit {
    fn new() -> Exit {
        Exit { locked: true }
    }

    fn open(&self) {
        if self.locked {
            println!("The door is locked");
        }
        else {
            println!("The door creaks open and sun pours into the room. You have escaped!");
        }
    }
}

impl Door {
    fn new() -> Door {
        Door { locked: true }
    }
    fn open(&self) -> bool {
        self.locked
    }
}

impl<'a> MazePath<'a> {
    fn new(additional_rooms: u32) -> MazePath<'a> {
        let max_rooms_attached = 3;
        // This needs to return a tuple at some point so key requirements can bubble back up and not be tied to the room they are in
        fn build<'a, 'b>(additional_rooms: u32, exit_here: bool, max_rooms_attached: &'a u32) -> MazePath<'b> {
            if additional_rooms == 0 {
                if exit_here {
                    println!("Exit here!");
                    MazePath::Exit{ entrance: Door::new(), exit: Exit::new(), containers: vec![] } // update when container generator is ready
                }
                else {
                    println!("made a room");
                    MazePath::Room { door: Door::new(), containers: vec![] }
                }
            }
            else {
                let attached_room_count = rand::thread_rng().gen_range(0, cmp::min(additional_rooms, *max_rooms_attached) + 1);
                let branch_with_exit = rand::thread_rng().gen_range(0, attached_room_count + 1); // gen_range is exclusive on the RHS, hence the + 1
                println!("Branch with exit: {}", branch_with_exit);
                let mut rooms_left = additional_rooms - attached_room_count;
                let mut rooms_here = 0;
                
                MazePath::Connector{ door: Door::new(), containers: vec![], other_rooms: {
                    let mut rooms = vec![];
                    println!("Attached room count: {}", attached_room_count);
                    for room_num in (0..attached_room_count + 1) {
                        println!("Rooms left to generate: {}", rooms_left);

                        if room_num == branch_with_exit { println!("Making exit branch") }
                        rooms_here = 
                            if room_num == attached_room_count {
                                println!("At end of connector branch, rooms_here: {}", rooms_left);
                                rooms_left  
                            }
                            else {
                                rand::thread_rng().gen_range(0, rooms_left + 1)
                            };
                            
                        rooms_left -= rooms_here;
                        rooms.push(build(rooms_here, exit_here && room_num == branch_with_exit, max_rooms_attached))
                    }
                    rooms 
                }}
            }
        }
        build(additional_rooms, true, &max_rooms_attached)
    }
    
}

impl<'a> Searchable for MazePath<'a> {
    fn search(&self) {
        let containers = match *self {
            MazePath::Room { containers: ref cs, .. } => cs,
            MazePath::Connector { containers: ref cs, .. } => cs,
            MazePath::Exit { containers: ref cs, .. } => cs
        };
        if containers.len() > 0 {
            println!("The room contains various items")
        }
        for container in containers {
            container.print_name()
        }
    }
}

impl<'a, 'b> Maze<'a, 'b> {
    pub fn new(num_rooms: u32, player_name: &'a str) -> Maze<'a, 'b> {
        Maze { start: InitialRoom, maze: MazePath::new(num_rooms), player: Player::new(player_name) }
    }

    pub fn help(&self) {
        println!("{}", "Commands:\n\tmove <room number or back> (move 0 and back are equivalent)\n\tinspect <item or room>".to_string() + 
                 "\n\tsearch <item or room>\n\tbreak <item>\n\thelp \n\tquit")
    }

    pub fn take_input(&mut self) -> bool {
        print!(">>> "); io::stdout().flush();
        let mut cmd = String::new();
        match io::stdin().read_line(&mut cmd) {
            Result::Err(..) => println!("Hm... that's odd. Try that command again"),
            Result::Ok(..) => {  }
        }
        //cmd.to_lowercase(); // this is an unstable feature in rust, nevertheless it would be nice to have in here
        let commands : Vec<&str> = cmd.split(|c: char| c == ' ' || c == '\n').collect();
        if commands.len() > 0 {
            for c in &commands {
                utils::printer(c);
            }
            match commands[0] {
                "quit" => { println!("You'll be back sooner or later"); true },
                "help" => { self.help(); false },
                s      => { println!("Try that one again, chief"); false }
            }
        }
        else {
            println!("Give me a command!");
            false 
        }
        
    }
    /*pub fn move_player(&mut self) {
        match self.player.pos {
            Option::None => 
        }
    }*/
}

