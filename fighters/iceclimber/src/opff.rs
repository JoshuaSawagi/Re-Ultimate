use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

pub extern "C" fn popo_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
    }
}

pub extern "C" fn nana_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
    }
}

pub fn install_popo(agent: &mut Agent) {
    agent.on_line(Main, popo_frame_wrapper);
}

pub fn install_nana(agent: &mut Agent) {
    agent.on_line(Main, nana_frame_wrapper);
}