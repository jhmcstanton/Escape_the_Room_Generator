extern crate rand;

use rand::Rng;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::convert::AsRef;

use traits::{Searchable, Breakable, Describable};
use items;
use utils;
use utils::{Either, Possibly, StringGenerator};

#[derive (Clone)]
pub enum Container {
    DurableSmall { name: String, description: String, item: Possibly<items::Item, items::Key> },
    FragileSmall { name: String, description: String, item: Possibly<items::Item, items::Key>, broken: bool, break_msg: String, broken_desc: String },
    Large { name: String, description: String, items: Vec<Either<items::Item, items::Key>> },//Vec<items::Item>, keys: Vec<items::Key> },
    Bed { name: String, description: String, item: Possibly<items::Item, items::Key> }, //Option<items::Item>, key: Option<items::Key> },
    Desk { name: String, description: String, computer: Option<Computer>, items: Vec<Either<items::Item, items::Key>> } //Vec<items::Item>, keys: Vec<items::Key>}
}

#[derive (Clone)]
pub struct Computer {
    name: String,
    desc: String,
    password: String,
    hint_url: String,
    id : u32,
    lock_opened: bool
}

pub struct ContainerStringGenerator {
    gen: HashMap<String, StringGenerator>
}



impl ContainerStringGenerator {
    // uses paths relative to the binary itself to make the generator
    pub fn new() -> ContainerStringGenerator {
        let base_path = "res/containers/";
        let folders   = vec![
            "bed/",
            "desk/",
           // "general/", //this is a special case 
            "large/",
            "small_durable/",
            "small_fragile/"];
        let files    = vec![
            "adjectives.txt",
            "nouns.txt",];
        let additional_files = vec!["break_msgs.txt", "broken_descs.txt", "general/adjectives.txt"];
            
        let mut generator   = ContainerStringGenerator { gen: HashMap::new() };
        for folder in folders {
            generator.gen.insert(folder.to_string(), StringGenerator::empty());
            for file_name in &files {
                let file_to_open = base_path.to_string() + &folder + file_name;
                
                let f = match File::open(&file_to_open) {
                    Ok(f) => f,
                    Err(e) => { utils::kill_with_file_error(&file_to_open, e); panic!("clean this up!") }
                };
                let mut file = BufReader::new(&f);
                for line in file.lines(){
                    match (line, file_name.to_string().as_ref()) {
                        (Err(e), _) => { utils::kill_with_file_error(&file_to_open, e); panic!("Clean this up!")},
                        (Ok(l), "adjectives.txt")  => {
                            if let Some(str_gen) = generator.gen.get_mut(folder) {
                                str_gen.feed(utils::WordClass::Adjective(l))
                            }
                        }
                        (Ok(l), "nouns.txt")       => {
                            if let Some(str_gen) = generator.gen.get_mut(folder) {
                                str_gen.feed(utils::WordClass::Noun(l))
                            }
                        }
                        (Ok(l), _ )                => {
                            println!("Filename: {} does not match expected file, contact developer", file_name);
                            panic!("Closing!")
                        }
                    }
                }
            }
            // need to add more resources so this can be done right
            for file_name in &additional_files {
                let file_to_open = base_path.to_string() + file_name;
                let f = match File::open(file_to_open) {
                    Err(e) => { utils::kill_with_file_error(file_name, e); panic!("Clean this up!") },
                    Ok(f)  => f 
                };
                let file = BufReader::new(&f);
                for line in file.lines(){
                    match (line, file_name.to_string().as_ref()) {
                        (Err(e), s )               => { utils::kill_with_file_error(s, e); panic!("Clean this up!")},
                        (Ok(l), "break_msgs.txt")  => {
                            if let Some(str_gen) = generator.gen.get_mut(folder) {
                                str_gen.feed(utils::WordClass::BreakMsg(l))
                            }
                        }
                        (Ok(l), "broken_descs.txt") => {
                            if let Some(str_gen) = generator.gen.get_mut(folder) {
                                str_gen.feed(utils::WordClass::BrokenDesc(l))
                            }
                        }
                        (Ok(l), "general/adjectives.txt") => {
                            if let Some(str_gen) = generator.gen.get_mut(folder) {
                                str_gen.feed(utils::WordClass::GenAdjective(l))
                            }
                        }
                        (Ok(l), s) => println!("Found odd file : {}", s)
                    }
                }
            }            
        }
        generator
    }
}

impl Container {
    pub fn generate() -> Vec<Container> {
        let max_num_containers = 5;
        let mut containers     = vec![];
        let str_generator  = ContainerStringGenerator::new();

        let num_containers = rand::thread_rng().gen_range(0, max_num_containers + 1);
        for _ in (0..num_containers) {
            containers.push(Container::from_num(&str_generator, rand::thread_rng().gen_range(0, 5)));
        }
        containers
    }

    /// This is useful when a container that can hold a computer or any number of items / keys is needed immediately and definitely
    pub fn mk_desk() -> Container {
        match ContainerStringGenerator::new().gen.get("desk/") {
            Option::Some(gen) => {
                let (name, desc) = gen.name_desc_pair();
                Container::Desk { name: name, description: desc, computer: Option::None, items: vec![] }
            }
            Option::None      => panic!("Could not directly construct desk object! Closing program")
        }
    }

    fn from_num(str_generator: &ContainerStringGenerator, n: u32) -> Container { 
        let container_types    = vec![
            "bed/",
            "desk/",
            //"general",
            "large/",
            "small_durable/",
            "small_fragile/"];
        let container_type = container_types[rand::thread_rng().gen_range(0, container_types.len())];
        match (str_generator.gen.get(container_type), container_type) {
            (Option::None, _)     => { println!("Container type found: {}", &container_type); panic!("String generator was not populated correctly!")}, 
            (Option::Some(g), "small_durable/") => { println!("Making small durable container");
                let (name, desc) = g.name_desc_pair();
                Container::DurableSmall {
                    name        : name,
                    description : desc,
                    item        : Option::None,
                }
            }
            (Option::Some(g), "small_fragile/")  => { println!("making small fragile container");
                let (name, desc) = g.name_desc_pair();
                let (broken_desc, break_msg) = g.broken_str_pair();
                Container::FragileSmall {
                    name        : name,
                    description : desc,
                    broken_desc : broken_desc,
                    item        : Option::None,
                    broken      : false,
                    break_msg   : break_msg
                }
            }
            (Option::Some(g), "large/") => { println!("making large container");
                let (name, desc) = g.name_desc_pair();
                Container::Large {
                    name        : name,
                    description : desc,
                    items : vec![]
                }
            }
            (Option::Some(g), "bed/")  => { println!("making bed");
                let (name, desc) = g.name_desc_pair();
                Container::Bed {
                    name        : name,
                    description : desc,
                    item        : Option::None
                }
            }
            (Option::Some(g), _) => { println!("making desk");
                let (name, desc) = g.name_desc_pair();
                Container::Desk {
                    name        : name,
                    description : desc,
                    computer    : Option::None,
                    items       : vec![]
                }
            }
        }
    }
}

impl Describable for Container {
    fn name(&self) -> String {
        match self {
            &Container::FragileSmall { name : ref name, .. } => name.to_string(),
            &Container::DurableSmall { name : ref name, .. } => name.to_string(),
            &Container::Large        { name : ref name, .. } => name.to_string(),
            &Container::Bed          { name : ref name, .. } => name.to_string(),
            &Container::Desk         { name : ref name, .. } => name.to_string()
        }
    }
    fn desc(&self) -> String {
        match self {
            &Container::DurableSmall { description: ref desc, .. } => desc.to_string(),
            &Container::Large        { description: ref desc, .. } => desc.to_string(),
            &Container::Bed          { description: ref desc, .. } => desc.to_string(),
            &Container::Desk         { description: ref desc, .. } => desc.to_string(),
            &Container::FragileSmall { description: ref desc, broken: ref broken, broken_desc: ref broken_desc, .. } => {
                if *broken { desc.to_string() } else { broken_desc.to_string() } 
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
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn desc(&self) -> String {
        self.desc.to_string()
    }
}

impl Searchable<utils::Either<items::Item, items::Key>> for Container {
    fn items(&self) -> Vec<utils::Either<items::Item, items::Key>> {
        match self {
            &Container::DurableSmall { item : ref op, .. } => {
                match op {
                    &None        => vec![],
                    &Some(ref i) => vec![i.clone()]
                }
            }
            &Container::FragileSmall { item : ref op, .. } => {
                match op {
                    &None        => vec![],
                    &Some(ref i) => vec![i.clone()],
                }
            }
            &Container::Bed { item: ref i, .. } => {
                match i {
                    &None => vec![],
                    &Some(ref i) => vec![i.clone()],                   
                }
            }
            &Container::Large { items: ref items, .. } => {
                items
                    .iter()
                    .map(|i| i.clone())
                    .collect()
            }
            &Container::Desk { items: ref items, .. } => {
                items
                    .iter()
                    .map(|i| i.clone())
                    .collect()
            }
        }
    }
}
