extern crate rand;
extern crate std; 

use rand::Rng;
use std::cmp;

pub struct Maze {
    start: InitialRoom,
    maze : MazePath
}

pub struct Door {
    locked: bool
}

pub struct Exit {
    locked: bool
}

pub struct InitialRoom;
//    exit    : Door, 
//}

pub enum MazePath{
    Room {
        door: Door,
    },
    Connector {
        door: Door,
        other_rooms: Vec<MazePath>
    },
    Exit {
        entrance: Door,
        exit: Exit,
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

impl MazePath {
    fn new(additional_rooms: u32) -> MazePath {
        let max_rooms_attached = 3;
        // This needs to return a tuple at some point so key requirements can bubble back up and not be tied to the room they are in
        fn build(additional_rooms: u32, exit_here: bool, max_rooms_attached: &u32) -> MazePath {
            if additional_rooms == 0 {
                if exit_here {
                    println!("Exit here!");
                    MazePath::Exit{ entrance: Door::new(), exit: Exit::new() }
                }
                else {
                    println!("made a room");
                    MazePath::Room { door: Door::new() }
                }
            }
            else {
                let attached_room_count = rand::thread_rng().gen_range(0, std::cmp::min(additional_rooms, *max_rooms_attached) + 1);
                let branch_with_exit = rand::thread_rng().gen_range(0, attached_room_count + 1); // gen_range is exclusive on the RHS, hence the + 1
                println!("Branch with exit: {}", branch_with_exit);
                let mut rooms_left = additional_rooms - attached_room_count;
                let mut rooms_here = 0;
                
                MazePath::Connector{ door: Door::new(), other_rooms: {
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

impl Maze {
    pub fn new(num_rooms: u32) -> Maze {
        Maze { start: InitialRoom, maze: MazePath::new(num_rooms) }
    }
}
