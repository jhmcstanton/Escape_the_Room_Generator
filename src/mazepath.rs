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
    id: u32,
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

pub enum MazePath<'a> {
    Room {
        door: Door,
        containers: Vec<containers::Container>,
        parent: Option<&'a MazePath<'a>>
    },
    Connector {
        door: Door,
        other_rooms: Vec<MazePath<'a>>,
        containers: Vec<containers::Container>,
        parent: Option<&'a MazePath<'a>>
    },
    Exit {
        entrance: Door,
        exit: Exit,
        containers: Vec<containers::Container>,
        parent: Option<&'a MazePath<'a>>
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
    fn locked(&self) -> bool {
        self.locked
    }
}

impl<'a> MazePath<'a> {
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
    
    pub fn new(additional_rooms: u32) -> (MazePath<'a>, Vec<items::Key>){
        let max_rooms_attached = 3;
        let key_here_chance = 50;
        let mut current_key = 0;
        let container_generator = containers::ContainerStringGenerator::new();        
        // This needs to return a tuple at some point so key requirements can bubble back up and not be tied to the room they are in
        // This feels waay too long. Maybe this can be broken into 3 functions: single room, exit, and connector constructors, with this function call them
        fn build<'a: 'b, 'b>(additional_rooms: u32, exit_here: bool, max_rooms_attached: &'b u32, key_here_chance: &'b u32, current_key:&'b mut u32, parent: Option<&'a MazePath<'a>>, keys_to_place: &'b mut Vec<items::Key>) -> MazePath<'a> {
            *current_key = *current_key + 1; // There is a minimum of 1 door to add when this function is called
            if additional_rooms == 0 {
                let mut containers = containers::Container::generate();
                let (door, door_key) = Door::new("Silver key".to_string(), "A simple key for a simple lock.".to_string(), *current_key);
                keys_to_place.push(door_key);
                
                if exit_here {
                    println!("Exit here!");
                    //let (door, door_key) = Door::new("".to_string(), "".to_string(), current_key + 1);
                    let (exit, exit_key) = Exit::new("Skeleton key.".to_string(), "A spooky skeleton key.".to_string(), *current_key + 2);
                    *current_key = *current_key + 1;
                    match MazePath::try_place_key(key_here_chance, exit_key, &mut containers) {
                        Some(k) => keys_to_place.push(k),
                        None    => ()
                    };
                    
                    println!("In exit made {} keys.", keys_to_place.len()); //expect 1 or 2
                    MazePath::Exit{ entrance: door, exit: exit, containers: containers::Container::generate(), parent: parent }
                }
                else {
                    println!("made a room");
                    println!("In room made {} keys.", keys_to_place.len()); // expect 1
                    MazePath::Room { door: door, containers: containers::Container::generate(), parent: parent }
                }
            }
            else {
                let (door, door_key) = Door::new("Brass key".to_string(), "A simple brass key".to_string(), *current_key);
                keys_to_place.push(door_key);
                let mut keys_to_add = vec![];     
                
                let mut connector = MazePath::Connector{ door: door, containers: containers::Container::generate(), parent: parent, other_rooms: vec![]};
                // need to insert reference to parents ! This should be a SOME not a NONE
                let other_rooms = mk_other_rooms(None, &additional_rooms, &max_rooms_attached, &key_here_chance, current_key, &exit_here, &mut keys_to_add);
                
                connector = match connector {
                    MazePath::Connector { door: door, containers: mut cs, parent: parent, .. } => {                        
                        for key in keys_to_add {
                            match MazePath::try_place_key(key_here_chance, key, &mut cs) {
                                Some(k) => { println!("Pushing key in previous room!"); keys_to_place.push(k)},
                                None    => println!("Placed key in connector!")
                            };
                        }
                        MazePath::Connector{ door: door, containers: cs, parent: parent, other_rooms: other_rooms }
                    }
                    _ => panic!("Something has gone horribly wrong in a pattern match inside mazepath::connector creation")
                };
                
                println!("Keys to place in previous rooms: {}", keys_to_place.len());
                connector
            }
        }

        fn mk_other_rooms<'a: 'b, 'b>(parent_connector: Option<&'a MazePath<'a>>, additional_rooms: &'b u32, max_rooms_attached: &'b u32, key_here_chance: &'b u32, current_key: &'b mut u32, exit_here: &'b bool, keys_to_add: &'b mut Vec<items::Key>) -> Vec<MazePath<'a>> {
            let attached_room_count = rand::thread_rng().gen_range(0, cmp::min(additional_rooms, max_rooms_attached) + 1);
            let branch_with_exit = rand::thread_rng().gen_range(0, attached_room_count + 1); // gen_range is exclusive on the RHS, hence the + 1
            println!("Branch with exit: {}", branch_with_exit);
            let mut rooms_left = additional_rooms - attached_room_count;
            let mut rooms_here = 0;
            
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
                let child_path = build(rooms_here
                                       , *exit_here && room_num == branch_with_exit
                                       , max_rooms_attached
                                       , key_here_chance
                                       , current_key
                                       , parent_connector
                                       , keys_to_add); 
                rooms.push(child_path);                                                
            }
            rooms
        }
        let mut keys = vec![];
        let path = build(additional_rooms, true, &max_rooms_attached, &key_here_chance, &mut current_key, None, &mut keys);
        println!("Num keys in initial room: {}", keys.len());
        (path, keys)
            
    }

    /// Rolls a 1d100 to see if the key is in this room, and places it here if possible. Otherwise, it returns None. 
    pub fn try_place_key<'b>(chance_key_here: &'b u32, key: items::Key, containers: &mut Vec<containers::Container>) -> Option<items::Key> {
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

impl<'a> Searchable<containers::Container> for MazePath<'a> {
    fn items(&self) -> Vec<containers::Container> {
        match self {
            &MazePath::Room { containers: ref cs, .. } => cs.clone(),
            &MazePath::Connector { containers: ref cs, .. } => cs.clone(),
            &MazePath::Exit { containers: ref cs, .. } => cs.clone()
        }
    }

}
