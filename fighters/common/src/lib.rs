#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]
#![allow(internal_features)]
#![feature(repr_simd)]
#![feature(simd_ffi)]
#![feature(core_intrinsics)]
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use smash::app::*;
use smash::app;
use smash::hash40;
use smash::app::sv_animcmd::*;
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;

#[macro_use] extern crate smash_script;

pub mod opff;
pub mod general_statuses;
pub mod function_hooks;
pub mod player_tags;
//pub mod misc;

pub static mut LAST_ATTACK_TEAM_COLOR: i32 = 0;

extern "C" fn common_init(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::common::instance::OCCUPIED_LEDGE_ID, -1);
    VarModule::set_int(fighter.battle_object, vars::common::instance::OCCUPIED_LEDGE_ID_FOR_TETHERS, -1);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_INIT);
    //unsafe { MATCH_SCOPED_RANDOM_U32 = app::sv_math::rand(hash40("fighter"), skyline::libc::c_int::MAX) as u32; };
}

pub fn install() {
    opff::install();
    general_statuses::install();
    function_hooks::install();
    player_tags::install();
    //misc::install();
    
    Agent::new("fighter")
        .on_start(common_init)
        .install();
}