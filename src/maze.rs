extern crate std;

use std::io;
use std::io::Write;

use utils;
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
        println!("{}", "Commands:\n\tmove <room number or back> (move 0 and back are equivalent)\n\tinspect <item or room>".to_string() + 
                 "\n\tsearch <item or room>\n\tbreak <item>\n\thelp \n\tquit") // Lol, I need to stop programming in the middle of the night
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
                s      => self.bad_cmd("Try that one again, chief")
            }
        }
        else {
            self.bad_cmd("Give me a command!")
        }
        
    }

    fn inspect(&self, commands: Vec<&str>) -> bool {
        if commands.len() > 1 {
            match commands[1] {
                "room" => {
                    println!("You see {} doors.", match self.player.pos {
                        None => 1,
                        Some(&MazePath::Exit{ .. } ) => 2,
                        Some(&MazePath::Room{ .. } ) => 1,
                        Some(&MazePath::Connector{ other_rooms: ref rs, .. }) => rs.len() + 1 
                    });
                    match self.player.pos {
                        Some(ref r) => r.search(),
                        None        => self.start.search()
                    }
                    
                    false
                }
                s => self.bad_cmd("Uh, this is weird. Try that again") 
            }
        }
        else {
            self.bad_cmd("Uh, what did you want to look at?")
        }
    }
    
    fn bad_cmd(&self, msg: &str) -> bool {
        utils::printer(msg);
        false
    }
    fn move_player(&mut self, room: Option<&'a MazePath>) {
        self.player.previous_room = self.player.pos;
        self.player.pos           = room;
    }
}
