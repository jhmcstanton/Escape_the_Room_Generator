use traits::{Searchable, Breakable, Describable};
use items;

pub enum Container<'a> {
    DurableSmall { name: &'a str, description: &'a str, item: items::Item<'a> },
    FragileSmall { name: &'a str, description: &'a str, item: items::Item<'a>, broken: bool, break_msg: &'a str, broken_desc: &'a str },
    Large { name: &'a str, description: &'a str, items: Vec<items::Item<'a>> },
    Bed { description: &'a str, item: items::Item<'a> },
}

pub struct Computer<'a> {
    name: &'a str,
    desc: &'a str,
    password: &'a str,
    hint_url: &'a str,
    lock_opened: bool
}

impl<'a> Describable for Container<'a> {
    fn name(&self) {
        match *self {
            Container::DurableSmall{ name: ref name, .. } => items::printer(name),
            Container::FragileSmall{ name: ref name, .. } => items::printer(name),
            Container::Large{ name: ref name, .. } => items::printer(name),
            Container::Bed{ .. } => ()
        }
    }

    fn describe(&self) {
        match *self {
            Container::DurableSmall{ description: ref desc, .. } => items::printer(desc),
            Container::Large{ description: ref desc, .. } => items::printer(desc),
            Container::Bed{ description: ref desc, .. } => items::printer(desc),
            Container::FragileSmall{ description: ref desc, broken_desc: ref broken_desc, broken: ref broken, .. } => {
                if *broken { items::printer(broken_desc) } else { items::printer(desc) }
            }
        }
    }
}

impl<'a> Breakable for Container<'a> {
    fn destroy(&mut self) {
        match *self {
            Container::FragileSmall { broken: ref mut broken, break_msg: ref mut msg, .. } => {
                if !*broken {
                    items::printer(msg);
                    *broken = true;
                }
                else {
                    println!("Thats already destroyed... What are you doing?")
                }
            }
            _ => println!("I don't think that will work..")
        }
    }
}

impl<'a> Describable for Computer<'a> {
    fn name(&self) {
        items::printer(self.name)
    }
    fn describe(&self) {
        items::printer(self.desc)
    }
}
