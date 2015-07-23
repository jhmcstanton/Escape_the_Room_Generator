extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt::Display;

use traits::{ Describable };

pub fn printer(str: &str) -> () {
    println!("{}", str)
}

pub fn kill_with_file_error<E: Display>(file_name: &str, error: E) {
    println!("Error opening file: {}, error: {}", file_name, error);
    println!("Possible cause: missing res folder (needs to be in same directory as game binary)");
    panic!("Closing program due to error");
}

// hand rolled either type since Result is semantically different
#[derive (Clone)]
pub enum Either<A, B> {
    Left(A),
    Right(B)
}

impl<A: Describable, B: Describable> Describable for Either<A, B> {
    fn name(&self) -> String{
        match self {
            &Either::Left(ref a)  => a.name(),
            &Either::Right(ref b) => b.name()
        }
    }
    fn desc(&self) -> String {
        match self {
            &Either::Left(ref a) => a.desc(),
            &Either::Right(ref b) => b.desc()
        }
    }
    /*
    fn print_name(&self) {
        match self {
            &Either::Left(ref a)  => a.print_name(),
            &Either::Right(ref b) => b.print_name()
        }
    }
    fn print_desc(&self) {
        match self {
            &Either::Left(ref a)  => a.print_desc(),
            &Either::Right(ref b) => b.print_desc()
        }
    }*/
}

pub type Possibly<A, B> = Option<Either<A, B>>;

pub enum Movement {
    Previous,
    Forward(u32)
}

pub enum WordClass {
    Noun(String),
    //Pronoun(String),
    //Verb(String),
    GenAdjective(String),
    Adjective(String),
    //Adverb(String),
    BrokenDesc(String),
    BreakMsg(String)
}

pub struct StringGenerator {
    nouns          : Vec<String>,
    //pronouns       : Vec<String>,
    //verbs          : Vec<String>,
    gen_adjectives : Vec<String>,
    adjectives     : Vec<String>,
    //adverbs        : Vec<String>,
    break_msgs     : Vec<String>,
    broken_descs   : Vec<String>
}

impl<'a> StringGenerator{
    
    pub fn name_desc_pair(&self) -> (String, String) {
        let noun    = self.nouns[rand::thread_rng().gen_range(0, self.nouns.len())].clone();
        let gen_adj = self.gen_adjectives[rand::thread_rng().gen_range(0, self.gen_adjectives.len())].clone();
        let adj     = self.adjectives[rand::thread_rng().gen_range(0, self.adjectives.len())].clone();
        let desc = "A ".to_string() + &gen_adj + ", " + &adj + " " + &noun;
        (noun, desc)
    }

    pub fn broken_str_pair(&self) -> (String, String) {
        let broken_desc = self.broken_descs[rand::thread_rng().gen_range(0, self.broken_descs.len())].clone();
        let break_msg   = self.break_msgs[rand::thread_rng().gen_range(0, self.break_msgs.len())].clone();
        (broken_desc, break_msg)
    }
    
    pub fn empty() -> StringGenerator {
        StringGenerator{
            nouns          : vec![],
            //pronouns       : vec![],
            //verbs          : vec![],
            adjectives     : vec![],
            //adverbs        : vec![],
            gen_adjectives : vec![],
            break_msgs     : vec![],
            broken_descs   : vec![]
        }
    }

    pub fn new(words: Vec<WordClass>) -> StringGenerator {
        let mut gen = StringGenerator::empty();
        for w in words {
            gen.feed(w);
        }
        gen
    }

    pub fn feed(&mut self, word: WordClass) {
        match word {
            WordClass::Noun(s)         => self.nouns.push(s),
            //WordClass::Pronoun(s)      => self.pronouns.push(s),
            //WordClass::Verb(s)         => self.verbs.push(s),
            WordClass::GenAdjective(s) => self.gen_adjectives.push(s),
            WordClass::Adjective(s)    => self.adjectives.push(s),
            //WordClass::Adverb(s)       => self.adverbs.push(s),
            WordClass::BreakMsg(s)     => self.break_msgs.push(s),
            WordClass::BrokenDesc(s)   => self.broken_descs.push(s)
        };
    }   
}

    
