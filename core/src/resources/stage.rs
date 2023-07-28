
use super::*;

use crate::sys::Stage;

pub struct CurrentStage {
    pub(crate) current: Stage
}

impl CurrentStage {
    pub fn get(&self) -> Stage {
        self.current
    }
}

impl CoreId for CurrentStage {
    fn id() -> usize {
        config::RES_LIMIT - 2
    }
}

impl Default for CurrentStage {
    fn default() -> Self {
        Self { current: Stage::Startup }
    }
}