use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn printer(str: &str) -> () {
    println!("{}", str)
}

// hand rolled either type since Result is semantically different
pub enum Either<A, B> {
    Left(A),
    Right(B)
}

pub enum Possibly<A, B> {
    Some(Either<A, B>),
    None
}

pub enum WordClass {
    Noun(String),
    Pronoun(String),
    Verb(String),
    GenAdjective(String),
    Adjective(String),
    Adverb(String),
    BrokenDesc(String),
    BreakMsg(String)
}

pub struct StringGenerator {
    nouns          : Vec<String>,
    pronouns       : Vec<String>,
    verbs          : Vec<String>,
    gen_adjectives : Vec<String>,
    adjectives     : Vec<String>,
    adverbs        : Vec<String>,
    break_msgs     : Vec<String>,
    broken_descs   : Vec<String>
}

impl<'a> StringGenerator{
   /*pub fn name_desc_pair(&self) -> (String, String) {
        
    }*/
    
    pub fn empty() -> StringGenerator {
        StringGenerator{
            nouns          : vec![],
            pronouns       : vec![],
            verbs          : vec![],
            adjectives     : vec![],
            adverbs        : vec![],
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
            WordClass::Pronoun(s)      => self.pronouns.push(s),
            WordClass::Verb(s)         => self.verbs.push(s),
            WordClass::GenAdjective(s) => self.gen_adjectives.push(s),
            WordClass::Adjective(s)    => self.adjectives.push(s),
            WordClass::Adverb(s)       => self.adverbs.push(s),
            WordClass::BreakMsg(s)     => self.break_msgs.push(s),
            WordClass::BrokenDesc(s)   => self.broken_descs.push(s)
        };
    }   
}

    
