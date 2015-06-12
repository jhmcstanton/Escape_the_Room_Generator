use items;
use maze;

pub struct Player<'a> {
    name: &'a str,
    keys: Vec<items::Item<'a>>,
    items: Vec<items::Item<'a>>,
    pos  : Option<&'a maze::MazePath<'a>>,
    previous_room: Option<&'a maze::MazePath<'a>>
}

impl<'a> Player<'a> {
    pub fn new<'b>(name: &'b str ) -> Player<'b> {
        Player{ name: name, keys: vec![], items: vec![], pos: Option::None, previous_room: Option::None }
    }
}
