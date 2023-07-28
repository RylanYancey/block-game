
use super::*;

pub struct Core {
    pub(crate) res: Res,
    pub(crate) sys: Systems,
}

impl Core {
    pub fn new() -> Self {
        Self {
            res: Res::new(),
            sys: Systems::new(),
        }
    }

    pub fn register_resource<T>(&mut self) 
        where T: CoreId + Default + 'static
    {
        self.res.data[T::id()] = Box::new(T::default());
    }

    pub fn register_system<F>(&mut self, stage: Stage, sys: F) 
        where F: Fn(&mut Core) + 'static
    {
        match stage {
            Stage::Startup => self.sys.startup_sys.push(Box::new(sys)),
            Stage::Title => self.sys.title_sys.push(Box::new(sys)),
            Stage::All => self.sys.all_sys.push(Box::new(sys)),
            Stage::Main => self.sys.main_sys.push(Box::new(sys)),
        }
    }
}