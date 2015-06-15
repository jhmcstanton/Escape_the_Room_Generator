extern crate markov;

use markov::Chain;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn printer(str: &str) -> () {
    println!("{}", str)
}

enum WordClass<'a> {
    Noun(&'a str),
    Pronoun(&'a str),
    Verb(&'a str),
    Adjective(&'a str),
    Adverb(&'a str)
}

struct SentenceGenerator {
    nouns      : Chain<String>,
    pronouns   : Chain<String>,
    verbs      : Chain<String>,
    adjectives : Chain<String>,
    adverbs    : Chain<String>
}

impl<'a> SentenceGenerator{
    pub fn empty() -> SentenceGenerator {
        SentenceGenerator{
            nouns: Chain::for_strings(),
            pronouns: Chain::for_strings(),
            verbs: Chain::for_strings(),
            adjectives: Chain::for_strings(),
            adverbs: Chain::for_strings()
        }
    }

    pub fn new(words: Vec<WordClass>) -> SentenceGenerator {
        let mut gen = SentenceGenerator::empty();
        for w in words {
            gen.feed(w);
        }
        gen
    }

    pub fn feed(&mut self, word: WordClass) {
        match word {
            WordClass::Noun(ref s)      => self.nouns.feed_str(s),
            WordClass::Pronoun(ref s)   => self.pronouns.feed_str(s),
            WordClass::Verb(ref s)      => self.verbs.feed_str(s),
            WordClass::Adjective(ref s) => self.adjectives.feed_str(s),
            WordClass::Adverb(ref s)    => self.adverbs.feed_str(s)
        };
    }   
}

    
