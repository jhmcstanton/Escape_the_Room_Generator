use traits::{Breakable, Describable};
use utils;

pub enum Item<'a> {
    Key { name: &'a str, description: &'a str, id: u32 },
    DurableItem { name: &'a str, description: &'a str },
    FragileItem { name: &'a str, description: &'a str, broken: bool, break_msg: &'a str, broken_desc: &'a str }
}

impl<'a> Describable for Item<'a> {
    fn print_name(&self) -> () {
        match self {
            &Item::Key{ name: ref name, .. } => utils::printer(name),
            &Item::DurableItem{ name: ref name, .. } => utils::printer(name),
            &Item::FragileItem{ name: ref name, .. } => utils::printer(name)
        }
    }
    fn print_desc(&self) -> () {
        match self {
            &Item::Key{ description: ref d, .. } => utils::printer(d),
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

