use items;

struct Player<'a> {
    name: &'a str,
    keys: Vec<items::Item<'a>>,
    items: Vec<items::Item<'a>>
}

impl<'a> Player<'a> {
    fn new<'b>(name: &'b str) -> Player<'b> {
        Player{ name: name, keys: vec![], items: vec![] }
    }
}
