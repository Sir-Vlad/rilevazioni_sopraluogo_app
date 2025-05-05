use std::any::Any;

pub trait ToList {
    fn to_list(&self) -> Vec<Box<dyn Any>>;
}
