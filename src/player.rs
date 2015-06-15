use traits::{Describable};
use items;
use maze;

pub struct Player<'a: 'b, 'b> {
    name: &'a str,
    keys: Vec<&'b items::Key<'a>>,
    items: Vec<&'b items::Item<'a>>,
    pos  : Option<&'b maze::MazePath<'a>>,
    previous_room: Option<&'b maze::MazePath<'a>>
}

impl<'a, 'b> Player<'a, 'b> {
    pub fn new(name: &'a str ) -> Player<'a, 'b> {
        Player{ name: name, keys: vec![], items: vec![], pos: Option::None, previous_room: Option::None }
    }

    pub fn traverse(&mut self, next_room: &'b maze::MazePath<'a>) {
        self.previous_room = self.pos;
        self.pos = Option::Some(next_room);
    }

    pub fn add_key(&mut self, key: &'b items::Key<'a>) {
        self.keys.push(key); /*
        match key {
            &items::Key::Key{ .. } => self.keys.push(key),
            _ => panic!("Cannot add item into key inventory!")
        } */
    }

    pub fn add_item(&mut self, item: &'b items::Item<'a>) {
        self.items.push(item);
        /*match item {
            &items::Item::Key { .. } => panic!("Keys cannot be added into item inventory, only key inventory"),
            _ => self.items.push(item)
        }*/
    }

    pub fn list_keys(&self) {
        for key in self.keys.iter() {
            key.print_name()
        }
    }

    pub fn list_items(&self) {
        for item in self.items.iter() {
            item.print_name()
        }
    }

    // Lists both items and keys, may be useful for the UI
    pub fn list_inventory(&self) {
        println!("Keys held: ");
        self.list_keys();
        println!("Items in inventory: ");
        self.list_items();
    }
}
