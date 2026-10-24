use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Hash40};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash::app::utility::*;
use vars::*;
use crate::sv_kinetic_energy;
use crate::globals::STICK_X;
use smash::app;

//static mut ECB_Y_OFFSETS: [f32; 9] = [0.0; 9];
pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

const PIVOT_STICK_SNAPBACK_WINDOW: f32 = 2.0;
unsafe fn pivots(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_value_x: f32, curr_frame: f32){

    // Get the pivot boost amount for the current character
    let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let mut pivot_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: dash_speed * 0.75, y: 0.0, z: 0.0};
    if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
        && curr_frame <= PIVOT_STICK_SNAPBACK_WINDOW && stick_value_x == 0.0
        && [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&StatusModule::prev_status_kind(boma, 0))
        && ![*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::prev_status_kind(boma, 1))
    {
        
        if curr_frame == 3.0 {
            pivot_boost.x = dash_speed * 0.35; // Reduce pivot speed boost if on the last frame of pivot leniency
        }
        
        PostureModule::reverse_lr(boma);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN,true);
        KineticModule::clear_speed_all(boma);
        KineticModule::add_speed(boma, &pivot_boost);
    }
}

unsafe fn double_shield_button_airdodge(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    // airdodge with second shield button while holding another shield button
    if boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_button_trigger(Buttons::GuardHold)
    && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
    && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_PASS,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3
    ])
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        return;
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32, curr_frame: f32) {
    double_shield_button_airdodge(boma, status_kind, situation_kind, cat[0]);

}