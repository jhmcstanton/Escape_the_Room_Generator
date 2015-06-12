
pub trait Describable {
    fn name(&self) ;
    fn describe (&self) ;
}

pub trait Breakable {
    fn destroy(&mut self);
}

pub trait Searchable {
    fn search(&self);
}
