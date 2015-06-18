pub trait Describable {
    fn print_name(&self) ;
    fn print_desc (&self) ;
}

pub trait Breakable {
    fn destroy(&mut self);
}

// intended for rooms or things containing container objects
pub trait Searchable<T: Describable> {
    fn items(&self) -> &Vec<T>;
    
    fn search(&self) {
        let items : &Vec<T> = self.items();
        if items.len() > 0 {
            println!("You see:")
        }
        for item in items {
            item.print_name()
        }
    }
}
