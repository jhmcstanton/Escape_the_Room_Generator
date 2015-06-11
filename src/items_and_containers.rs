
pub enum Item {
    Key { name: &str, description: &str, id: u32 },
    DurableItem { name: &str, description: &str },
    FragileItem { name: &str, description: &str, broken: bool, break_msg: &str, broken_desc: &str }
}

pub enum Container {
    DurableSmall { name: &str, description: &str, item: Item },
    FragileSmall { name: &str, description: &str, item: Item, broken: bool, break_msg: &str, broken_desc: &str },
    Large { name: &str, description: &str, items: Vec<Item> },
    Bed { description: &str, item: Item },
}

trait Describable {
    fn describe (&self) -> ();
}
trait Breakable {
    fn break(&mut self) -> ();
}

impl Describable for Item {
    fn describe(&self) -> () {
        match self {
            &Item::Key{ name: ref n, description: ref d } => println!(d),
            &Item::DurableItem{ name: ref n, description: ref d } => println!(d),
            &Item::FragileItem{
                name: ref n,
                description: ref desc,
                broken: ref broken,
                break_msg: ref msg,
                broken_desc: ref broken_desc } => if broken { println!(broken_desc) } else { println!(description) }
        }
    }
}
