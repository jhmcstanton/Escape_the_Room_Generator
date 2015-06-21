use traits::{Breakable, Describable};
use utils;

#[derive (Clone)]
pub enum Item {
    DurableItem { name: String, description: String },
    FragileItem { name: String, description: String, broken: bool, break_msg: String, broken_desc: String }
}

#[derive (Clone)]
pub enum Key {
    Key { name: String, description: String, id: u32 },
    Password { word: String, used: bool }  // not currently used, may never be used like this
}

impl Describable for Key {
    fn name(&self) -> String {
        match self {
            &Key::Key{ name: ref name, .. }   => name.to_string(),
            &Key::Password{ word: ref w, .. } => w.to_string() // maybe don't expose this API outside of testing
        }
    }
    fn desc(&self) -> String {
        match self {
            &Key::Key{ description: ref name, .. } => name.to_string(),
            &Key::Password{ word: ref w, .. }      => w.to_string()  // maybe don't expose this API outside of testing
        }
    }
}

impl Describable for Item {
    fn name(&self) -> String {
        match self {
            &Item::DurableItem{ name: ref name, .. } => "WTH".to_string(),//utils::printer(name),
            &Item::FragileItem{ name: ref name, .. } => "UGH".to_string() //utils::printer(name)
        }
    }
    fn desc(&self) -> String {
        match self {
            &Item::DurableItem{ description: ref d, .. } => d.to_string(), 
            &Item::FragileItem{ description: ref desc, broken: ref broken, broken_desc: ref broken_desc, .. } =>
                if *broken { desc.to_string() } else { desc.to_string() }
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

