extern crate rand;

use rand::Rng;

use traits::{Searchable, Breakable, Describable};
use items;
use utils;
use utils::{Either, Possibly, StringGenerator};

pub enum Container {
    DurableSmall { name: String, description: String, item: Possibly<items::Key, items::Item> },
    FragileSmall { name: String, description: String, item: Possibly<items::Key, items::Item>, broken: bool, break_msg: String, broken_desc: String },
    Large { name: String, description: String, items: Vec<items::Item>, keys: Vec<items::Key> },
    Bed { description: String, item: Option<items::Item>, key: Option<items::Key> },
    Desk { description: String, computer: Option<Computer>, items: Vec<items::Item>, keys: Vec<items::Key>}
}

pub struct Computer {
    name: String,
    desc: String,
    password: String,
    hint_url: String,
    id : u32,
    lock_opened: bool
}

pub struct ContainerStringGenerator {
    durable_small_gen : StringGenerator,
    fragile_small_gen : StringGenerator,
    large_gen         : StringGenerator,
    bed_gen           : StringGenerator,
    desk_gen          : StringGenerator
}

impl Container {
    pub fn generate(str_generator: ContainerStringGenerator) -> Vec<Container> {
        let max_num_containers = 5;
        let mut containers = vec![];

        let num_containers = rand::thread_rng().gen_range(0, max_num_containers + 1);
        for _ in (0..num_containers) {
            containers.push(Container::from_num(&str_generator, rand::thread_rng().gen_range(0, 5)));
        }
        containers
    }

    fn from_num(str_generator: &ContainerStringGenerator, n: u32) -> Container { // need to make a markov generator for containers still
        
        match n {
            0 =>{
                let (name, desc) = str_generator.durable_small_gen.name_desc_pair();
                Container::DurableSmall {
                    name        : name,
                    description : desc,
                    item        : Possibly::None,
                }
            }
            1 =>{
                let (name, desc) = str_generator.fragile_small_gen.name_desc_pair();
                let (broken_desc, break_msg) = str_generator.fragile_small_gen.broken_str_pair();
                Container::FragileSmall {
                    name        : name,
                    description : desc,
                    broken_desc : broken_desc,
                    item        : Possibly::None,
                    broken      : false,
                    break_msg   : break_msg
                }
            }
            2 => {
                let (name, desc) = str_generator.large_gen.name_desc_pair();
                Container::Large {
                    name        : name,
                    description : desc,
                    items : vec![],
                    keys  : vec![]
                }
            }
            3 => {
                let (_, desc) = str_generator.bed_gen.name_desc_pair();
                Container::Bed {
                    description : desc,
                    item        : Option::None,
                    key         : Option::None
                }
            }
            _ => {
                let (_, desc) = str_generator.desk_gen.name_desc_pair();
                Container::Desk {
                    description : desc,
                    computer    : Option::None,
                    items       : vec![],
                    keys        : vec![]
                }
            }
        }
    }

    /*b fn add_key(&mut self, key : items::Key) {
        match self {
            &Container::DurableSmall{ 
        }
    }*/
}

impl Describable for Container {
    fn print_name(&self) {
        match *self {
            Container::DurableSmall{ name: ref name, .. } => utils::printer(name),
            Container::FragileSmall{ name: ref name, .. } => utils::printer(name),
            Container::Large{ name: ref name, .. } => utils::printer(name),
            Container::Bed{ .. } => println!("Who names a bed?"),
            Container::Desk{ .. } => println!("You try naming a desk, ok? It's not easy")
        }
    }

    fn print_desc(&self) {
        match *self {
            Container::DurableSmall{ description: ref desc, .. } => utils::printer(desc),
            Container::Large{ description: ref desc, .. } => utils::printer(desc),
            Container::Bed{ description: ref desc, .. } => utils::printer(desc),
            Container::Desk{ description: ref desc, .. } => utils::printer(desc),
            Container::FragileSmall{ description: ref desc, broken_desc: ref broken_desc, broken: ref broken, .. } => {
                if *broken { utils::printer(broken_desc) } else { utils::printer(desc) }
            }
        }
    }
}

impl Breakable for Container {
    fn destroy(&mut self) {
        match *self {
            Container::FragileSmall { broken: ref mut broken, break_msg: ref mut msg, .. } => {
                if !*broken {
                    utils::printer(msg);
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

impl Describable for Computer {
    fn print_name(&self) {
        utils::printer(&self.name)
    }
    fn print_desc(&self) {
        utils::printer(&self.desc)
    }
}
