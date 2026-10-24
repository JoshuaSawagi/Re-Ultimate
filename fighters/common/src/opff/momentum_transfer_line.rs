use utils::{
    *,
    ext::*,
    consts::*,
	consts::globals::*
};
use smash_script::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;

//===================================================================
//== MOMENTUM TRANSFER HELPER
//== Performs some extra calculations to help with momentum handling
//===================================================================
pub unsafe fn waveland_transfer_helper(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, curr_frame: f32) {
	if situation_kind == *SITUATION_KIND_AIR && ![*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) {
		VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET);
	}
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    let curr_frame = MotionModule::frame(boma);
    waveland_transfer_helper(fighter, lua_state, l2c_agent, boma, status_kind, situation_kind, fighter_kind, curr_frame);
}