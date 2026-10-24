use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn up_special_reverse(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    //Lucina frame 6
    let mut target_frame = 3.0;
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if frame == target_frame {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    up_special_reverse(boma, status_kind, stick_x, facing, frame);
}

pub extern "C" fn roy_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		roy_frame(fighter);
    }
}

pub unsafe fn roy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, roy_frame_wrapper);
}