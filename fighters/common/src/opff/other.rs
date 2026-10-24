use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::phx::*;
use smash::app::sv_animcmd::*;
use smash::app::smashball::*;
use smash_script::*;
use globals::*;
use crate::util::get_fighter_common_from_accessor;
use smash::cpp::root::app::SituationKind;
use smash::hash40;
use smash::phx::Hash40;
use vars::*;
use crate::StatusModule::status_kind;
use crate::app::utility::get_kind;
use crate::StatusModule::situation_kind;
use super::*;

// Shifts Run, RunBrake, TurnRun, and TurnRunBrake animations to match any horizontal hip bone adjustment to vanilla dash animations
// otherwise the animations don't transition properly into one another
// This is so we don't have to edit those 4 other animations if we want to edit a dash anim
unsafe fn custom_dash_anim_support(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_RUN) && fighter.is_motion(Hash40::new("run")) {
        let dash_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X);
        let run_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::RUN_HIP_OFFSET_X);
        let mut hip_translate = Vector3f::zero();
        MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
        hip_translate.z += dash_hip_offset_x - run_hip_offset_x;
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);
    }
    else if fighter.is_prev_status(*FIGHTER_STATUS_KIND_RUN)
    && StatusModule::is_changing(fighter.module_accessor)
    && !fighter.is_status(*FIGHTER_STATUS_KIND_TURN_RUN) {
        ModelModule::clear_joint_srt(fighter.module_accessor, Hash40::new("hip"));
    }
    
    if fighter.is_status(*FIGHTER_STATUS_KIND_TURN_RUN) && fighter.is_motion(Hash40::new("turn_run")) {
        let dash_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X);
        let run_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::RUN_HIP_OFFSET_X);
        let mut hip_translate = Vector3f::zero();
        MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
        hip_translate.z += dash_hip_offset_x - run_hip_offset_x;
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);
    }
    else if fighter.is_prev_status(*FIGHTER_STATUS_KIND_TURN_RUN)
    && StatusModule::is_changing(fighter.module_accessor)
    && !fighter.is_status(*FIGHTER_STATUS_KIND_RUN) {
        ModelModule::clear_joint_srt(fighter.module_accessor, Hash40::new("hip"));
    }
}

pub extern "C" fn left_stick_flick_counter(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.left_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X, u8::MAX as i32 - 1);
        } else if fighter.left_stick_x().signum() != fighter.prev_left_stick_x().signum() || fighter.prev_left_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X);
        }
        
        if fighter.left_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y, u8::MAX as i32 - 1);
        } else if fighter.left_stick_y().signum() != fighter.prev_left_stick_y().signum()
        || fighter.prev_left_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y);
        }
    }
}

pub extern "C" fn right_stick_flick_counter(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.right_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X, u8::MAX as i32 - 1);
        } else if fighter.right_stick_x().signum() != fighter.prev_right_stick_x().signum() || fighter.prev_right_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X);
        }
        
        if fighter.right_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y, u8::MAX as i32 - 1);
        } else if fighter.right_stick_y().signum() != fighter.prev_right_stick_y().signum() || fighter.prev_right_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y);
        }
    }
}

pub unsafe fn ecb_shift_disabled_motions(fighter: &mut L2CFighterCommon) {
    if ( (fighter.kind() == *FIGHTER_KIND_SZEROSUIT
            && fighter.is_motion(Hash40::new("attack_air_hi")))
        || (fighter.kind() == *FIGHTER_KIND_PALUTENA
            && fighter.is_motion(Hash40::new("attack_air_n")))
        || (fighter.kind() == *FIGHTER_KIND_GANON
            && fighter.is_motion_one_of(&[Hash40::new("attack_air_n"), Hash40::new("attack_air_lw"), Hash40::new("attack_air_hi")])) )
        
    && !VarModule::is_flag(fighter.battle_object, vars::common::status::DISABLE_ECB_SHIFT)
    {
        VarModule::on_flag(fighter.battle_object, vars::common::status::DISABLE_ECB_SHIFT);
    }
}

pub unsafe fn cliff_xlu_frame_counter(fighter: &mut L2CFighterCommon) {
    let cliff_xlu = VarModule::get_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME);
    // If you have ledge intan frames left
    if cliff_xlu > 0 {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            // Remove ledge intan on landing
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
            VarModule::set_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME, 0);
        }
        else{
            VarModule::dec_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME);
        }
    }
}

// Teeter cancelling
pub unsafe fn teeter_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if (boma.is_situation(*SITUATION_KIND_GROUND)
        && boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_RUN_BRAKE,
            *FIGHTER_STATUS_KIND_APPEAL,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        ])
        && ((KineticModule::get_sum_speed_x(
            fighter.module_accessor,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL,
        ) - KineticModule::get_sum_speed_x(
            fighter.module_accessor,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND,
        ) - KineticModule::get_sum_speed_x(
            fighter.module_accessor,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN,
        )) * PostureModule::lr(boma))
            > 0.0)
    {
        // Conditions for transitioning to teeter animation in sub_ground_check_ottotto
        if (GroundModule::is_ottotto(boma, 1.72) // Original value: 0.86
        && fighter.left_stick_x().abs() < 0.75)
        {
            fighter.change_status(FIGHTER_STATUS_KIND_OTTOTTO.into(), true.into());
        }
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    left_stick_flick_counter(fighter);
    right_stick_flick_counter(fighter);
    cliff_xlu_frame_counter(fighter);
    custom_dash_anim_support(fighter);
    ecb_shift_disabled_motions(fighter);
    teeter_cancel(fighter, boma);
}
