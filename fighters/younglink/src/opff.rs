use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn fire_arrow_drift(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_BOUND
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}


pub extern "C" fn younglink_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        younglink_frame(fighter);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    fire_arrow_drift(fighter, boma, status_kind, situation_kind, cat[1], stick_y);
    fastfall_specials(fighter);
}

pub unsafe fn younglink_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, younglink_frame_wrapper);
}