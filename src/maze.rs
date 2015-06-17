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
use utils::{Either};
use traits::{Describable, Searchable, Breakable};
use player::Player;

pub struct Maze<'a> {
    start: InitialRoom,
    maze : MazePath,
    player: Player<'a>
}

pub struct Door {
    locked: bool,
    id : u32,
}

pub struct Exit {
    locked: bool,
    id: u32
}

pub struct InitialRoom{
    containers: Vec<containers::Container>,
}

impl InitialRoom {
    pub fn new(keys: Vec<items::Key>) -> InitialRoom {
        let mut containers = containers::Container::generate();
        // forcing a desk *should* make all keys insert as expected, since it can hold any number of keys
        containers.push(containers::Container::mk_desk()); 
        for key in keys {
            match MazePath::try_place_key(&0, key, &mut containers) {
                Some(k) => panic!("Unable to play all keys in initial room! Please restart!"),
                None    => ()
            };
        }
        InitialRoom { containers: containers }
    }
}

pub enum MazePath {
    Room {
        door: Door,
        containers: Vec<containers::Container>
    },
    Connector {
        door: Door,
        other_rooms: Vec<MazePath>,
        containers: Vec<containers::Container>
    },
    Exit {
        entrance: Door,
        exit: Exit,
        containers: Vec<containers::Container>
    }
}

impl Exit {
    fn new<'a>(key_name: String, key_desc: String, key_id: u32) -> (Exit, items::Key) {
        ( Exit { locked: true , id: key_id}, items::Key::Key{name: key_name, description: key_desc, id: key_id})
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
    fn new<'a>(key_name: String, key_desc: String, key_id: u32) -> (Door, items::Key) { 
        (Door { locked: true, id: 0 }, items::Key::Key{name: key_name, description: key_desc, id: key_id})
    }
    fn open(&self) -> bool {
        self.locked
    }
}

impl MazePath {
    fn new(additional_rooms: u32) -> (MazePath, Vec<items::Key>){
        let max_rooms_attached = 3;
        let key_here_chance = 50;
        let current_key = 0;
        let container_generator = containers::ContainerStringGenerator::new();
        // This needs to return a tuple at some point so key requirements can bubble back up and not be tied to the room they are in
        fn build<'a>(additional_rooms: u32, exit_here: bool, max_rooms_attached: &'a u32, key_here_chance: &'a u32, current_key: u32) -> (MazePath, u32, Vec<items::Key>) {
            if additional_rooms == 0 {
                let mut containers = containers::Container::generate();
                let mut keys_to_place = vec![];
                let (door, door_key) = Door::new("".to_string(), "".to_string(), current_key);
                match MazePath::try_place_key(key_here_chance, door_key, &mut containers) {
                    Some(k) => keys_to_place.push(k),
                    None    => ()
                };
                
                if exit_here {
                    println!("Exit here!");
                    let (door, door_key) = Door::new("".to_string(), "".to_string(), current_key + 1);
                    let (exit, exit_key) = Exit::new("".to_string(), "".to_string(), current_key + 2);
                    
                    match MazePath::try_place_key(key_here_chance, exit_key, &mut containers) {
                        Some(k) => keys_to_place.push(k),
                        None    => ()
                    };
                    
                    (MazePath::Exit{ entrance: door, exit: exit, containers: containers::Container::generate() }
                     , current_key + 2, keys_to_place)
                }
                else {
                    println!("made a room");

                    (MazePath::Room { door: door, containers: containers::Container::generate() }, current_key + 1, keys_to_place)
                }
            }
            else {
                let attached_room_count = rand::thread_rng().gen_range(0, cmp::min(additional_rooms, *max_rooms_attached) + 1);
                let branch_with_exit = rand::thread_rng().gen_range(0, attached_room_count + 1); // gen_range is exclusive on the RHS, hence the + 1
                println!("Branch with exit: {}", branch_with_exit);
                let mut rooms_left = additional_rooms - attached_room_count;
                let mut rooms_here = 0;
                let mut keys_to_add = vec![];

                let (door, door_key) = Door::new("".to_string(), "".to_string(), current_key);
                let mut current_key  = current_key + 1;
                let connector = MazePath::Connector{ door: door, containers: vec![], other_rooms: {
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
                        match build(rooms_here, exit_here && room_num == branch_with_exit, max_rooms_attached, key_here_chance, current_key) {
                            (room, key_id, keys) => {
                                rooms.push(room);
                                current_key = key_id;
                                for key in keys {
                                    keys_to_add.push(key);
                                }
                            }
                        }
                        //rooms.push(build(rooms_here, exit_here && room_num == branch_with_exit, max_rooms_attached, current_key))
                    }
                    rooms
                }};
                (connector, current_key, keys_to_add)
            }
        }
        let (path, _, keys) = build(additional_rooms, true, &max_rooms_attached, &key_here_chance, current_key);
        (path, keys)
    }

    /// Rolls a 1d100 to see if the key is in this room, and places it here if possible. Otherwise, it returns None. 
    pub fn try_place_key(chance_key_here: &u32, key: items::Key, containers: &mut Vec<containers::Container>) -> Option<items::Key> {
        let p : u32 = rand::thread_rng().gen_range(0, 100); 
        if  p < *chance_key_here {
            for container in containers {
                match container {
                    &mut containers::Container::DurableSmall{ item: ref mut i, .. } => {
                        match i {
                            &mut Option::Some(_) => (),
                            _ => i = { Option::Some(key); return Option::None }
                        }                        
                    }
                    &mut containers::Container::FragileSmall { item: ref mut i, .. } => {
                        match i {
                            &mut Option::Some(_) => (),
                            _ => i = { Option::Some(key); return Option::None } 
                        }
                    }
                    &mut containers::Container::Bed { key: ref mut k, .. } => {
                        match k {
                            &mut Option::Some(_) => (),
                            _               => key = { Option::Some(k); return Option::None }
                        }
                    }
                    &mut containers::Container::Large { keys: ref mut ks, .. } => {
                        ks.push(key);
                        return Option::None
                    }
                    &mut containers::Container::Desk { keys: ref mut ks, .. } => {
                        ks.push(key);
                        return Option::None
                    }
                }
            }
            Option::None            
        }
        else {
            Option::None
        }
    }
    
}

impl Searchable for InitialRoom {
    fn containers<'a: 'b, 'b>(&'a self) -> &'b Vec<containers::Container> {
        &self.containers
    }
}

impl Searchable for MazePath {
    fn containers<'a: 'b, 'b>(&'a self) -> &'b Vec<containers::Container> {
        match self {
            &MazePath::Room { containers: ref cs, .. } => cs,
            &MazePath::Connector { containers: ref cs, .. } => cs,
            &MazePath::Exit { containers: ref cs, .. } => cs
        }
    }
}

impl<'a> Maze<'a> {
    pub fn new(num_rooms: u32, player_name: String) -> Maze<'a> {
        let (maze_path, keys) = MazePath::new(num_rooms);
        Maze { start: InitialRoom::new(keys), maze: maze_path, player: Player::new(player_name) }
    }

    pub fn help(&self) {
        println!("{}", "Commands:\n\tmove <room number or back> (move 0 and back are equivalent)\n\tinspect <item or room>".to_string() + 
                 "\n\tsearch <item or room>\n\tbreak <item>\n\thelp \n\tquit") // Lol, I need to stop programming in the middle of the night
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
            match commands[0] {
                "quit" => { println!("You'll be back sooner or later"); true },
                "help" => { self.help(); false },
                "inspect" => { // this is really gros..
                    if commands.len() > 1 {
                        match commands[1] {
                            "room" => {
                                match self.player.pos {
                                    Some(ref r) => r.search(),
                                    None        => self.start.search()
                                }
                                false
                            }
                            _ => { println!("Uh... you see nothing!"); false } 
                        }
                    }
                    else { println!("Uh, what did you want to look at?"); false }                                                           
                }
                s      => { println!("Try that one again, chief"); false }
            }
        }
        else {
            println!("Give me a command!");
            false 
        }
        
    }

    fn move_player(&mut self, room: Option<&'a MazePath>) {
        self.player.previous_room = self.player.pos;
        self.player.pos           = room;
    }
}

