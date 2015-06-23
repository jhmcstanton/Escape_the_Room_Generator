extern crate std;

use std::io;
use std::io::Write;

use utils;
use utils::{ Either };
use mazepath::MazePath;
use mazepath::InitialRoom;
use player::Player;
use traits::{Searchable, Describable, Breakable};

pub struct Maze<'a> {
    start: InitialRoom,
    maze : MazePath,
    player: Player<'a>
}


impl<'a> Maze<'a> {
    pub fn new(num_rooms: u32, player_name: String) -> Maze<'a> {
        let (maze_path, keys) = MazePath::new(num_rooms);
        Maze { start: InitialRoom::new(keys), maze: maze_path, player: Player::new(player_name) }
    }

    pub fn help(&self) {
        println!("{}", "Commands:\n\tmove <room number or back> (move 0 and back are equivalent)\n\tinspect <container or room>".to_string() + 
                 "\n\ttake <container>;<item>\n\tbreak <item>\n\thelp \n\tquit") // Lol, I need to stop programming in the middle of the night
    }

    pub fn take_input(&mut self) -> bool {
        print!(">>> "); io::stdout().flush();
        let mut cmd = String::new();
        match io::stdin().read_line(&mut cmd) {
            Result::Err(..) => println!("Hm... that's odd. Try that command again"),
            Result::Ok(..) => {  }
        }
        //cmd.to_lowercase(); // this is an unstable feature in rust, nevertheless it would be nice to have in here
        let commands : Vec<&str> = cmd.split(|c: char| c == ' ' || c == '\n').collect();
        if commands.len() > 0 {
            match commands[0] {
                "quit" => { println!("You'll be back sooner or later"); true },
                "help" => { self.help(); false },
                "inspect" => { // this is really gross..
                    self.inspect(commands)  
                }
                "take"    => self.take(commands),
                s      => self.bad_cmd("Try that one again, chief")
            }
        }
        else {
            self.bad_cmd("Give me a command!")
        }
        
    }

    fn inspect(&self, mut commands: Vec<&str>) -> bool {
        if commands.len() > 1 {
            match commands[1] {
                "room" => {
                    println!("You see {} doors.", match self.player.pos {
                        None => 1,
                        Some(&mut MazePath::Exit{ .. } ) => 2,
                        Some(&mut MazePath::Room{ .. } ) => 1,
                        Some(&mut MazePath::Connector{ other_rooms: ref rs, .. }) => rs.len() + 1 
                    });
                    match self.player.pos {
                        Some(ref r) => r.search(),
                        None        => self.start.search()
                    }
                    
                    false
                }
                _ => {
                    let item_name = self.fold_cmds(commands);
                    
                    let cs = match self.player.pos {
                        None            => self.start.items(),
                        Some(ref r) => r.items()
                    };
                    for c in &cs {
                        if c.name() == item_name.trim() {
                            c.search();
                            break
                        }
                    }
                    false
                }
            }
        }
        else {
            self.bad_cmd("Uh, what did you want to look at?")
        }
    }
    
    fn fold_cmds(&self, mut commands: Vec<&str>) -> String {
        commands.remove(0);
        let mut init_str = commands.remove(0);
        commands.iter().fold(init_str.to_string(), |acc, w| acc + " " + w) // this seems like a super backwards way of doing this        
    }
    
    fn take(&mut self, cmds: Vec<&str>) -> bool {
        let command = self.fold_cmds(cmds);
        let tmp_iter : Vec<_>= command.split(";").collect();        
        let (container, item_name) = (tmp_iter[0].to_string(), tmp_iter[1].to_string());
        println!("Container: {}, item: {}", container, item_name);
        let item = match self.player.pos {
            None    => self.start.take_from(container, item_name),
            Some(ref mut r) => r.take_from(container, item_name) // cannot borrow mutably and immutably :/
        };
        match item {
            Some(Either::Left(item)) => self.player.add_item(item),
            Some(Either::Right(key)) => self.player.add_key(key),
            None                     => ()
        }
        false
    }
    
    fn bad_cmd(&self, msg: &str) -> bool {
        utils::printer(msg);
        false
    }
    fn move_player(&'a mut self, room: Option<&'a mut MazePath>) {
        self.player.traverse(room);
    }
}
