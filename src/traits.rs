use containers;

pub trait Describable {
    fn print_name(&self) ;
    fn print_desc (&self) ;
}

pub trait Breakable {
    fn destroy(&mut self);
}

// intended for rooms or things containing container objects
pub trait Searchable {
    fn containers<'a: 'b, 'b>(&'a self) -> &'b Vec<containers::Container>;
    
    fn search(&self) {
        let containers = self.containers();
        if containers.len() > 0 {
            println!("The room contains various items")
        }
        for container in containers {
            container.print_name()
        }
    }
}
