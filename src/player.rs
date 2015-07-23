use traits::{Describable};
use items;
use utils::Movement;
use mazepath;

use std::mem::swap;

pub struct Player<'a> {
    pub name: String,
    keys: Vec<items::Key>,
    items: Vec<items::Item>,
    pub pos: Option<&'a mut mazepath::MazePath>,
    pub previous_room: Option<&'a mut mazepath::MazePath>
}

impl<'a> Player<'a> {
    pub fn new(name: String ) -> Player<'a> {
        Player{ name: name, keys: vec![], items: vec![], pos: Option::None, previous_room: Option::None }
    }

    pub fn traverse(&'a mut self, movement: Movement) {
        swap(&mut self.previous_room, &mut self.pos);
        match movement {
            Movement::Previous            => (),
            Movement::Forward(room_index) => {
                match self.previous_room {
                    Some(&mut mazepath::MazePath::Connector{ other_rooms: ref mut rooms, .. }) => {
                        if rooms.len() > (room_index as usize) && room_index >= 0 {
                            self.pos = Some(&mut rooms[room_index as usize])
                        }
                        else {
                            panic!("Invalid room index! Bad check by caller to player.traverse")
                        }
                    }
                    _ => {
                        panic!("Incorrect room for indexing in player.traverse!")
                    }
                }
            }
        }
    }
    
/*    pub fn traverse(&'a mut self, next_room: Option<&'a mut mazepath::MazePath>) {
       swap(&mut self.previous_room, &mut self.pos); 
       self.pos = next_room;

    }*/

    pub fn add_key(&mut self, key: items::Key) {
        self.keys.push(key); 
    }

    pub fn add_item(&mut self, item: items::Item) {
        self.items.push(item);
    }

    pub fn list_keys(&self) {
        let _: Vec<_> = self.keys.iter().map(|k| k.print_name()).collect();        
    }

    pub fn list_items(&self) {
        let _: Vec<_> = self.items.iter().map(|i| i.print_name()).collect();
    }

    // Lists both items and keys, may be useful for the UI
    pub fn list_inventory(&self) {
        println!("Keys held: ");
        self.list_keys();
        println!("Items in inventory: ");
        self.list_items();
    }
}
