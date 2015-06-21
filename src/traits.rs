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

    /*
    There is no good way to do this with the existing function for items. If it returned a reference to a vector 
    (not actually possible for all structs in use) then this would be easy, otherwise this is not going to be writable.
    possible solution: 
    rewrite structs for containers to use an either and a single vector for keys and items
     */
    /*
    fn take(&mut self, item_name: String) -> T { 
        
    }*/
}
