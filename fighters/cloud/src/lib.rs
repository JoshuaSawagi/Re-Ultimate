#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod opff;
pub mod acmd;
use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use smash_script::macros::ATTACK_ABS;
use utils::{
    *,
    consts::*,
    ext::*
};
use smashline::*;
#[macro_use] extern crate smash_script;

pub fn install() {
    let agent = &mut Agent::new("cloud");
    opff::install(agent);
    acmd::install(agent);
    agent.install();
}