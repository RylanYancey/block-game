
mod input; // Index -1
mod stage; // Index -2

use super::*;

pub use input::Input;
pub use stage::CurrentStage;

pub(crate) fn load_core_resources(core: &mut Core) {
    core.register_resource::<Input>();
    core.register_resource::<CurrentStage>();
}