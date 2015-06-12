use traits::{Breakable, Describable};

pub enum Item<'a> {
    Key { name: &'a str, description: &'a str, id: u32 },
    DurableItem { name: &'a str, description: &'a str },
    FragileItem { name: &'a str, description: &'a str, broken: bool, break_msg: &'a str, broken_desc: &'a str }
}

impl<'a> Describable for Item<'a> {
    fn name(&self) -> () {
        match self {
            &Item::Key{ name: ref name, .. } => printer(name),
            &Item::DurableItem{ name: ref name, .. } => printer(name),
            &Item::FragileItem{ name: ref name, .. } => printer(name)
        }
    }
    fn describe(&self) -> () {
        match self {
            &Item::Key{ description: ref d, .. } => printer(d),
            &Item::DurableItem{ description: ref d, .. } => printer(d), 
            &Item::FragileItem{ description: ref desc, broken: ref broken, broken_desc: ref broken_desc, .. } =>
                if *broken { printer(broken_desc) } else { printer(desc) }
        }
    }
}

impl<'a> Breakable for Item<'a> {
    fn destroy(&mut self) -> () {
        match *self {
            Item::FragileItem{ break_msg: ref msg, broken: ref mut broken, ..} => {
                if !*broken {
                    printer(msg);
                    *broken = true;
                }
            }
            _  => println!("Hm, this seems too sturdy to break")
        }
    }
}

pub fn printer(str: &str) -> () {
    println!("{}", str)
}
