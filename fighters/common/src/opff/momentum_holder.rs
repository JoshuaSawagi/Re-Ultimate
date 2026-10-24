use smash::lib::{L2CValue, L2CAgent};
use skyline::nro::{self, NroInfo};
use smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::utility::get_kind;
use smash::app::utility::*;
use smash::lua2cpp::*;
use smash::lib::lua_const::*;
use smash::phx::*;
use smash::app::*;
use smash::app;
use skyline::nn::ro::LookupSymbol;
use smashline::*;
use super::*;
use smash::app::FighterKineticEnergyGravity;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smashline::Pre;
use globals::*;
use crate::function_hooks::controls;
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};

use skyline::install_hooks;

use super::*;
use rand::prelude::SliceRandom;
use rand::Rng;

pub static mut CURRENTMOMENTUM: [f32; 9] = [0.0; 9];

pub const SUB_STATUS: i32 = 0x15; //sub status, i32 value
pub const STICK_X: i32 = 0x1A; //stick x, f32 value
pub const MODULE_ACCESSOR: i32 = 0x5; //module accessor, ptr value
pub const NONE_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x: 0.0, y: 0.0, z: 0.0};
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRIED: i32 = 0x921;


pub unsafe fn returnSmall(arg1: f32, arg2: f32) -> f32{
    if arg1 < arg2 {
        return arg1;
    }
    else {
        return arg2;
    }
}

pub unsafe fn returnLarge(arg1: f32, arg2: f32) -> f32{
    if arg1 > arg2 {
        return arg1;
    }
    else {
        return arg2;
    }
}

pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

pub unsafe fn calcMomentum(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_mul = WorkModule::get_param_float(boma, hash40("jump_speed_x_mul"), 0);
    let stick_x = ControlModule::get_stick_x(boma);
    let x_vel = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let calcJumpSpeed = (jump_speed_x * stick_x) + (jump_speed_x_mul * x_vel);
    let mut jumpSpeedClamped = 0.0;
    if x_vel < 0.0 {
        jumpSpeedClamped = returnLarge(calcJumpSpeed, -1.0 * jump_speed_x_max);
    }
    else {
        jumpSpeedClamped = returnSmall(calcJumpSpeed, jump_speed_x_max);
    }
    jumpSpeedClamped
}

pub unsafe fn additionalTransfer(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_FALL].contains(&status_kind) {
        CURRENTMOMENTUM[get_player_number(boma)] = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); 
    }

    /*      ADDITIONAL MOVES THAT SHOULD CONSERVE MOMENTUM       */
    let mut should_conserve_momentum = false;
    if situation_kind == *SITUATION_KIND_AIR && MotionModule::frame(boma) <= 1.0 {

        if [*FIGHTER_KIND_MARIO, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_CAPTAIN, 
            *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_PIKACHU, 
            *FIGHTER_KIND_PICHU, *FIGHTER_KIND_GANON]
            .contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { //put any fighter here whose neutral special should conserve momentum
                should_conserve_momentum = true; 
        }

        if should_conserve_momentum && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs() > 0.1 {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(CURRENTMOMENTUM[get_player_number(boma)]));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }
    }
}

pub unsafe extern "C" fn run(fighter : &mut L2CFighterCommon) {
    JostleModule::set_team(fighter.module_accessor, 0);
    let lua_state = fighter.lua_state_agent;
    let mut l2c_agent = L2CAgent::new(lua_state);
    let module_accessor = &mut *fighter.module_accessor;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = get_kind(module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
    let cat3 = ControlModule::get_command_flag_cat(fighter.module_accessor, 2);
    let stick_value_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_value_x = ControlModule::get_stick_x(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let curr_frame = MotionModule::frame(fighter.module_accessor);
    let kinetic_type = KineticModule::get_kinetic_type(fighter.module_accessor);
    
    //Held Buffer
    let control_pad = [
        *CONTROL_PAD_BUTTON_APPEAL_HI, *CONTROL_PAD_BUTTON_APPEAL_LW, *CONTROL_PAD_BUTTON_APPEAL_S_L, *CONTROL_PAD_BUTTON_APPEAL_S_R, *CONTROL_PAD_BUTTON_ATTACK, *CONTROL_PAD_BUTTON_ATTACK_RAW, *CONTROL_PAD_BUTTON_CATCH, *CONTROL_PAD_BUTTON_CSTICK_ON,
        *CONTROL_PAD_BUTTON_FLICK_JUMP, *CONTROL_PAD_BUTTON_GUARD, *CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_INVALID, *CONTROL_PAD_BUTTON_JUMP, *CONTROL_PAD_BUTTON_JUMP_MINI, *CONTROL_PAD_BUTTON_SMASH, *CONTROL_PAD_BUTTON_SPECIAL, 
        *CONTROL_PAD_BUTTON_SPECIAL_RAW, *CONTROL_PAD_BUTTON_SPECIAL_RAW2, *CONTROL_PAD_BUTTON_STOCK_SHARE, *CONTROL_PAD_BUTTON_TERM, *CONTROL_PAD_CLATTER_CAUSE_NONE, *CONTROL_PAD_CLATTER_FLOWER, *CONTROL_PAD_CLATTER_MAIN, *CONTROL_PAD_CLATTER_NONE,
        *CONTROL_PAD_CLATTER_TERM, *CONTROL_PAD_STICK_REVERSE_ALL, *CONTROL_PAD_STICK_REVERSE_NONE, *CONTROL_PAD_STICK_REVERSE_X, *CONTROL_PAD_STICK_REVERSE_Y
    ];
    for i in control_pad {
        if ControlModule::get_trigger_count(module_accessor, i as u8) > 15 && ControlModule::check_button_on(module_accessor, i)
        && ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) {
            ControlModule::reset_trigger(module_accessor);
            ControlModule::clear_command(module_accessor, true);
        }
    }

    additionalTransfer(lua_state, &mut l2c_agent, module_accessor, status_kind, situation_kind, fighter_kind);

}