extern crate rand;

use rand::Rng;
use std::cmp;

fn main() {
    println!("Hello, world!");
}

struct Maze {
    start: InitialRoom,
    end  : InitialRoom,
    maze : InitialRoom
}

struct Door {
    locked: bool
}

struct Exit {
    locked: bool
}

struct FinalRoom {
    entrance: Door,
    escape  : Door,
}

struct InitialRoom {
    exit    : Door,
}

enum MazeRoom<'a> {
    Room {
        door: Door,
        previous_room: Option<&'a MazeRoom<'a>>
    },
    Connector {
        door: Door,
        previous_room : Option<&'a MazeRoom<'a>>,
        other_rooms: Vec<MazeRoom<'a>>
    },
    Exit {
        entrance: Door,
        exit: Exit,
        previous_room : Option<&'a MazeRoom<'a>>
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

impl<'a> MazeRoom<'a> {
    fn new(additional_rooms: &mut u32) -> MazeRoom<'a> {
        let max_rooms_attached = 3;
        
        let mut root = MazeRoom::Connector{ door: Door::new(), other_rooms = vec![], parent: Option::None };
        if additional_rooms <= 1 {
            return root;
        }
        let mut room_ptr = &root; 
        loop {
            if additional_rooms == 0{
                break;
            }
            if additional_rooms == 0 {
                
            }
            else {
                let num_attached_rooms = rand::thread_rng().gen_range(0,
            }

        }
        root
    }
    /*fn new(additional_rooms: u32) -> MazeRoom<'a> {
        let max_rooms_attached = 3;
        // This needs to return a tuple at some point so key requirements can bubble back up
        fn build<'a>(additional_rooms: u32, parent: Option<&'a MazeRoom>, exit_here: bool, max_rooms_attached: &u32) -> MazeRoom<'a> {
            if additional_rooms == 0 {
                if exit_here {
                    MazeRoom::Exit{ entrance: Door::new(), exit: Exit::new(), previous_room: parent }
                }
                else {
                    MazeRoom::Room { door: Door::new(), previous_room: parent }
                }
            }
            else {
                let attached_room_count = rand::thread_rng().gen_range(1, std::cmp::min(additional_rooms, max_rooms_attached + 1));
                let branch_with_exit = rand::thread_rng().gen_range(0, attached_room_count + 1); // gen_range is exclusive on the RHS, hence the + 1
                let mut rooms : Vec<MazeRoom> = vec![]; 
                let mut rooms_left = additional_rooms - attached_room_count;
                let mut rooms_here = 0;
                
                let this = MazeRoom::Connector{ door: Door::new(), previous_room: parent, other_rooms: rooms };
                                               
                for room_num in (0..attached_room_count + 1) {
                    rooms_here = rand::thread_rng().gen_range(0, rooms_left - (attached_room_count + 1));
                    rooms_left -= rooms_here;
                    rooms.push(build(rooms_here, Option::Some(&this), exit_here && room_num == branch_with_exit, max_rooms_attached))
                }
                this
            }
        }
        build(additional_rooms, Option::None, true, &max_rooms_attached)
    }*/
    
}


