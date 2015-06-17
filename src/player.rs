use traits::{Describable};
use items;
use maze;

pub struct Player<'a> {
    name: String,
    keys: Vec<&'a items::Key>,
    items: Vec<&'a items::Item>,
    pos  : Option<&'a maze::MazePath>,
    previous_room: Option<&'a maze::MazePath>
}

impl<'a> Player<'a> {
    pub fn new(name: String ) -> Player<'a> {
        Player{ name: name, keys: vec![], items: vec![], pos: Option::None, previous_room: Option::None }
    }

    pub fn traverse(&mut self, next_room: &'a maze::MazePath) {
        self.previous_room = self.pos;
        self.pos = Option::Some(next_room);
    }

    pub fn add_key(&mut self, key: &'a items::Key) {
        self.keys.push(key); 
    }

    pub fn add_item(&mut self, item: &'a items::Item) {
        self.items.push(item);
    }

    pub fn list_keys(&self) {
        self.keys.iter().map(|k| k.print_name());
    }

    pub fn list_items(&self) {
        self.items.iter().map(|i| i.print_name());
    }

    // Lists both items and keys, may be useful for the UI
    pub fn list_inventory(&self) {
        println!("Keys held: ");
        self.list_keys();
        println!("Items in inventory: ");
        self.list_items();
    }
}
