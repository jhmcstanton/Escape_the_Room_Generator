extern crate rand;
extern crate std;

use rand::Rng;
use std::cmp;
use std::io;
use std::io::Write;

use items;
use containers;
use utils;
use utils::{Either, Possibly};
use traits::{Describable, Searchable, Breakable};
use player::Player;


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
//            loop {
                match MazePath::try_place_key(&75, key, &mut containers) {
                    Some(_) => panic!("Unable to place all keys in initial room! Please restart!"),
                    None    => ()
                };
  //          }
        }
        InitialRoom { containers: containers }
    }

    pub fn take_from(&mut self, container_name: &str, item_name: &str) ->  Possibly<items::Item, items::Key> {
        for container in &mut self.containers {
            if &container.name()  == container_name {
                return container.take(item_name)
            }
        }
        None
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
    pub fn new<'a>(key_name: String, key_desc: String, key_id: u32) -> (Exit, items::Key) {
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
    pub fn new<'a>(key_name: String, key_desc: String, key_id: u32) -> (Door, items::Key) { 
        (Door { locked: true, id: 0 }, items::Key::Key{name: key_name, description: key_desc, id: key_id})
    }
    fn open(&self) -> bool {
        self.locked
    }
}

impl Describable for Door {
    fn name(&self) -> String {
        if self.locked {
            "A wooden door.".to_string()
        }
        else {
            "A wooden door (Accessed previously).".to_string()
        }
            
    }
    fn desc(&self) -> String {
        "Wooden and sturdy.".to_string()
    }
}

impl MazePath {
    pub fn door(&self) -> &Door {
        match *self {
            MazePath::Connector { door: ref door, .. } => door,
            MazePath::Room { door: ref door, .. } => door,
            MazePath::Exit { entrance: ref door, .. } => door
        }
    }
    pub fn containers(&self) -> &Vec<containers::Container> {
        match self {
            &MazePath::Connector { containers: ref cs, .. } => cs,
            &MazePath::Room      { containers: ref cs, .. } => cs,
            &MazePath::Exit      { containers: ref cs, .. } => cs
        }
    }

    fn mut_containers(&mut self) -> &mut Vec<containers::Container> {
        match self {
            &mut MazePath::Connector { containers: ref mut cs, .. } => cs,
            &mut MazePath::Room      { containers: ref mut cs, .. } => cs,
            &mut MazePath::Exit      { containers: ref mut cs, .. } => cs
        }
    }

    pub fn take_from(&mut self, container_name: &str, item_name: &str) -> Possibly<items::Item, items::Key> {
        for container in self.mut_containers() {
            if &container.name()  == container_name {
                return container.take(item_name)
            }
        }
        None
    }
    
    pub fn new(additional_rooms: u32) -> (MazePath, Vec<items::Key>){
        let max_rooms_attached = 3;
        let key_here_chance = 50;
        let current_key = 0;
        let container_generator = containers::ContainerStringGenerator::new();
        // This needs to return a tuple at some point so key requirements can bubble back up and not be tied to the room they are in
        // This feels waay too long. Maybe this can be broken into 3 functions: single room, exit, and connector constructors, with this function call them
        fn build<'a>(additional_rooms: u32, exit_here: bool, max_rooms_attached: &'a u32, key_here_chance: &'a u32, current_key: u32) -> (MazePath, u32, Vec<items::Key>) {
            if additional_rooms == 0 {
                let mut containers = containers::Container::generate();
                let (door, door_key) = Door::new("Silver key".to_string(), "A simple key for a simple lock.".to_string(), current_key);
                let mut keys_to_place = vec![door_key];
                
                if exit_here {
                    println!("Exit here!");
                    //let (door, door_key) = Door::new("".to_string(), "".to_string(), current_key + 1);
                    let (exit, exit_key) = Exit::new("Skeleton key.".to_string(), "A spooky skeleton key.".to_string(), current_key + 2);
                    
                    match MazePath::try_place_key(key_here_chance, exit_key, &mut containers) {
                        Some(k) => keys_to_place.push(k),
                        None    => ()
                    };
                    
                    println!("In exit made {} keys.", keys_to_place.len()); //expect 1 or 2
                    (MazePath::Exit{ entrance: door, exit: exit, containers: containers::Container::generate() }
                     , current_key + 2, keys_to_place)
                }
                else {
                    println!("made a room");
                    println!("In room made {} keys.", keys_to_place.len()); // expect 1
                    (MazePath::Room { door: door, containers: containers::Container::generate() }, current_key + 1, keys_to_place)
                }
            }
            else {
                let attached_room_count = rand::thread_rng().gen_range(0, cmp::min(additional_rooms, *max_rooms_attached) + 1);
                let branch_with_exit = rand::thread_rng().gen_range(0, attached_room_count + 1); // gen_range is exclusive on the RHS, hence the + 1
                println!("Branch with exit: {}", branch_with_exit);
                let mut rooms_left = additional_rooms - attached_room_count;
                let mut rooms_here = 0;

                let (door, door_key) = Door::new("Brass key".to_string(), "A simple brass key".to_string(), current_key);
                let mut keys_to_place = vec![door_key]; // used when returning from this function call
                let mut keys_to_add = vec![];           // used to collect keys 'bubbled' up from other build calls
                let mut current_key  = current_key + 1;
                
                let mut connector = MazePath::Connector{ door: door, containers: containers::Container::generate(), other_rooms: {
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
                        let (room, key_id, keys) = build(rooms_here
                                                         , exit_here && room_num == branch_with_exit
                                                         , max_rooms_attached
                                                         , key_here_chance
                                                         , current_key); 
                        rooms.push(room);
                        current_key = key_id;
                        for key in keys {
                            println!("Found key! pushing to keys_to_add!");
                            
                            keys_to_add.push(key);
                        }                                                
                        //rooms.push(build(rooms_here, exit_here && room_num == branch_with_exit, max_rooms_attached, current_key))
                    }
                    rooms
                }};
                match connector {
                    MazePath::Connector { containers: ref mut cs, .. } => {
                        for key in keys_to_add {
                            match MazePath::try_place_key(key_here_chance, key, cs) {
                                Some(k) => { println!("Pushing key in previous room!"); keys_to_place.push(k)},
                                None    => println!("Placed key in connector!")
                            };
                        }
                    }
                    _ => panic!("Something has gone horribly wrong in a pattern match insize mazepath")
                };
                
                println!("Keys to place in previous rooms: {}", keys_to_place.len());
                (connector, current_key, keys_to_place)
            }
        }
        let (path, _, keys) = build(additional_rooms, true, &max_rooms_attached, &key_here_chance, current_key);
        println!("Num keys in initial room: {}", keys.len());
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
                            _ =>  { *i = Option::Some(Either::Right(key)); return Option::None }
                        }                        
                    }
                    &mut containers::Container::FragileSmall { item: ref mut i, .. } => {
                        match i {
                            &mut Option::Some(_) => (),
                            _ =>  { *i = Option::Some(Either::Right(key)); return Option::None } 
                        }
                    }
                    &mut containers::Container::Bed { item: ref mut k, .. } => {
                        match k {
                            &mut Option::Some(_) => (),
                            _               => { *k = Option::Some(utils::Either::Right(key)); return Option::None }
                        }
                    }
                    &mut containers::Container::Large { items: ref mut ks, .. } => {
                        ks.push(Either::Right(key));
                        return Option::None
                    }
                    &mut containers::Container::Desk { items: ref mut ks, .. } => {
                        ks.push(Either::Right(key));
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

impl Searchable<containers::Container> for InitialRoom {
    fn items(&self) -> Vec<containers::Container> {
        self.containers.clone() 
    }    
}

impl Searchable<containers::Container> for MazePath {
    fn items(&self) -> Vec<containers::Container> {
        match self {
            &MazePath::Room { containers: ref cs, .. } => cs.clone(),
            &MazePath::Connector { containers: ref cs, .. } => cs.clone(),
            &MazePath::Exit { containers: ref cs, .. } => cs.clone()
        }
    }

}
