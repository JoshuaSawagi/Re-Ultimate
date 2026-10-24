
use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);
/*
unsafe fn air_falcon_kick_jump_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END
    ])
    && fighter.get_num_used_jumps() == fighter.get_jump_count_max()
    {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
}
*/
unsafe fn repeated_falcon_punch_turnaround(fighter: &mut L2CFighterCommon) {
    let frame = fighter.motion_frame();
    if fighter.is_status(*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN)
    && 22.0 < frame && frame < 41.0
    && fighter.is_stick_backward()
    && fighter.stick_x().abs() > 0.1
    {
        fighter.change_status_req(*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, true);
    }
}
/*
unsafe fn yes(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("attack_air_f")) && (14..17).contains(&(fighter.motion_frame() as u32)) {
        if !VarModule::is_flag(fighter.object(), vars::captain::status::YES) && fighter.is_button_on(Buttons::AppealAll) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            PLAY_SE(fighter, Hash40::new("vc_captain_appeal03"));
            VarModule::on_flag(fighter.object(), vars::captain::status::YES);
        }
    }
}
*/
pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {

    //air_falcon_kick_jump_reset(fighter);
    repeated_falcon_punch_turnaround(fighter);
    //yes(fighter);
}

pub extern "C" fn captain_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);

        captain_frame(fighter)
    }
}

pub unsafe fn captain_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, captain_frame_wrapper);
}