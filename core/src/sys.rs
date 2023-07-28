
use super::*;

pub struct Systems {
    pub(crate) startup_sys: Vec<Box<dyn Fn(&mut Core)>>,
    pub(crate) title_sys: Vec<Box<dyn Fn(&mut Core)>>,
    pub(crate) all_sys: Vec<Box<dyn Fn(&mut Core)>>,
    pub(crate) main_sys: Vec<Box<dyn Fn(&mut Core)>>,
}

impl Systems {
    pub fn new() -> Self {
        Self {
            startup_sys: Vec::new(),
            title_sys: Vec::new(),
            all_sys: Vec::new(),
            main_sys: Vec::new(),
        }
    }

    pub fn start(&self, core: &mut Core) {
        for f in self.startup_sys.iter() {
            f(core)
        }
    }

    pub fn run(&self, core: &mut Core) {
        for f in self.all_sys.iter() {
            f(core)
        }

        match core.res.get::<CurrentStage>().as_ref().current {
            Stage::Title => {
                for f in self.title_sys.iter() {
                    f(core)
                }
            },
            Stage::Main => {
                for f in self.title_sys.iter() {
                    f(core)
                }
            }
            _ => { panic!("Unexpected Error in Systems.run") }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Stage {
    Startup,
    Title,
    All,
    Main,
}