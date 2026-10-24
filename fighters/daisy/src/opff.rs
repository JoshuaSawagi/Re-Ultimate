
use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

pub extern "C" fn daisy_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
            common::opff::fighter_common_opff(fighter);
            //daisy_frame(fighter)
    }
}
/*
pub unsafe fn daisy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);    
    }
}
*/
pub fn install(agent: &mut Agent) {
    agent.on_line(Main, daisy_frame_wrapper);
}