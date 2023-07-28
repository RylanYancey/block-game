
use std::any::Any;

use super::CoreId;
use super::config::*;

pub struct Res {
    pub(crate) data: Vec<Box<dyn Any>>
}

impl Res {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(RES_LIMIT),
        }
    }

    pub fn get_mut<T>(&mut self) -> &mut Box<T> 
        where T: CoreId + 'static
    {
        match self.data[T::id()].downcast_mut::<Box<T>>() {
            Some(val) => {
                val
            }
            None => {
                panic!("Attempted to get_mut a resource, but the types did not match!")
            }
        }
    }

    pub fn get<T>(&self) -> &Box<T> 
        where T: CoreId + 'static
    {
        match self.data[T::id()].downcast_ref::<Box<T>>() {
            Some(val) => {
                val
            }
            None => {
                panic!("Attempted to get a resource, but the types did not match!")
            }
        }
    }
}


