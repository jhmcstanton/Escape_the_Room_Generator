use utils;

pub trait Describable {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn print_name(&self) {
        utils::printer(&self.name())
    }
    fn print_desc (&self) {
        utils::printer(&self.desc())
    }
}

pub trait Breakable {
    fn destroy(&mut self);
}

// intended for rooms or things containing container objects
pub trait Searchable<T: Describable> {
    fn items(&self) -> Vec<T>;
    
    fn search(&self) {
        let items : Vec<T> = self.items();
        if items.len() > 0 {
            println!("You see:");
            for item in items {
                item.print_name()
            }                
        }
        else {
            println!("There is nothing here.")
        }
    }

}
