use traits::{Breakable, Describable};
use utils;

pub enum Item<'a> {
    DurableItem { name: &'a str, description: &'a str },
    FragileItem { name: &'a str, description: &'a str, broken: bool, break_msg: &'a str, broken_desc: &'a str }
}

pub enum Key<'a> {
    Key { name: &'a str, description: &'a str, id: u32 },
    Password { id: u32, used: bool } 
}

impl<'a> Describable for Key<'a> {
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

impl<'a> Describable for Item<'a> {
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

impl<'a> Breakable for Item<'a> {
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

