
pub trait Describable {
    fn print_name(&self) ;
    fn print_desc (&self) ;
}

pub trait Breakable {
    fn destroy(&mut self);
}

pub trait Searchable {
    fn search(&self);
}
