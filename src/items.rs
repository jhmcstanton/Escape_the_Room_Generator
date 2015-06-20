use std::marker::Copy;

use traits::{Breakable, Describable};
use utils;


pub enum Item {
    #[derive (Copy)]
    DurableItem { name: String, description: String },
    #[derive (Copy)]
    FragileItem { name: String, description: String, broken: bool, break_msg: String, broken_desc: String }
}

pub enum Key {
    #[derive (Copy)]
    Key { name: String, description: String, id: u32 },
    #[derive (Copy)]
    Password { id: u32, used: bool } 
}

impl<'a> Describable for Key {
    fn print_name(&self) { 
        match self {
            &Key::Key{ name: ref name, .. } => utils::printer(name),
            &Key::Password{ id: ref w, .. } => println!("The password is: {}", w)// maybe don't expose this API outside of testing
        }
    }
    fn print_desc(&self) {
        match self {
            &Key::Key{ description: ref d, .. } => utils::printer(d),
            &Key::Password{ id: ref w, .. }     => println!("The password is: {}", w) // maybe don't expose this API outside of testing
        }
    }
}

impl Describable for Item {
    fn print_name(&self) -> () {
        match self {
            &Item::DurableItem{ name: ref name, .. } => utils::printer(name),
            &Item::FragileItem{ name: ref name, .. } => utils::printer(name)
        }
    }
    fn print_desc(&self) -> () {
        match self {
            &Item::DurableItem{ description: ref d, .. } => utils::printer(d), 
            &Item::FragileItem{ description: ref desc, broken: ref broken, broken_desc: ref broken_desc, .. } =>
                if *broken { utils::printer(broken_desc) } else { utils::printer(desc) }
        }
    }
}

impl Breakable for Item {
    fn destroy(&mut self) -> () {
        match *self {
            Item::FragileItem{ break_msg: ref msg, broken: ref mut broken, ..} => {
                if !*broken {
                    utils::printer(msg);
                    *broken = true;
                }
            }
            _  => println!("Hm, this seems too sturdy to break")
        }
    }
}

