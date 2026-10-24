
use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

pub extern "C" fn buddy_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, buddy_frame_wrapper);
}