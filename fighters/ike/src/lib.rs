#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;

use std::f32::consts::PI;
use smash::{
    lib::{
        L2CValue,
        LuaConst,
        *
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait,
            get_value_float
        },
        utility,
        lua_bind::{
            *,
            Article,
            KineticEnergy,
            KineticEnergyNormal,
            FighterKineticEnergyController,
            FighterManager,
            FighterInformation,
            FighterCutInManager
        }    
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::{
        *,
        Hash40,
        Vector2f,
        Vector3f
    },

};
use smash_script::{
    *,
    macros::*
};
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;
#[macro_use] extern crate smash_script;

pub fn install() {
    let agent = &mut Agent::new("ike");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}