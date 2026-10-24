use super::*;
use crate::guard::int;

// Credit to Wuboy for Guard_Off status
/* Yeah this section is to remove parry*/

#[skyline::hook(replace = L2CFighterCommon_status_pre_GuardOff)]
unsafe extern "C" fn status_pre_guardoff(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_initStatus)]
unsafe extern "C" fn sub_ftstatusuniqprocessguardoff_initstatus(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Common)]
unsafe extern "C" fn status_guardoff_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let enabled_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        // Updated transition terms
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
    ];
    for val in enabled_terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    let guard_off_cancel_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_cancel_frame"));
    WorkModule::set_int(fighter.module_accessor, guard_off_cancel_frame, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
    let motion_rate = 1.0;
    let guard_off_enable_shield_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_enable_shield_frame"));
    let disable_guard_escape_frame = guard_off_cancel_frame + guard_off_enable_shield_frame;
    WorkModule::set_int(fighter.module_accessor, disable_guard_escape_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
    WorkModule::set_int(fighter.module_accessor, disable_guard_escape_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_guard_off_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_guard_off_uniq as *const () as _));
    motion_rate.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_off_uniq)]
unsafe extern "C" fn sub_guard_off_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME) > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME) == 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_cancel)]
unsafe extern "C" fn sub_status_guard_off_main_common_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        let cat1_reserve = VarModule::get_int(fighter.battle_object, int::GUARD_OFF_RESERVE_CAT1);
        if cat1_reserve != 0 {
            let cat1 = fighter.global_table[CMD_CAT1].get_i32() | cat1_reserve;
            fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
            VarModule::set_int(fighter.battle_object, int::GUARD_OFF_RESERVE_CAT1, 0);
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into();
        }
    }

    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_control)]
unsafe extern "C" fn sub_status_guard_off_main_common_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if GroundModule::is_passable_ground(fighter.module_accessor)
    && fighter.global_table[STICK_Y].get_f32() <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"))
    && fighter.global_table[FLICK_Y].get_i32() < 3
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return true.into();
    }

    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_GuardOff)]
unsafe extern "C" fn status_end_guardoff(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_exitStatus)]
unsafe extern "C" fn sub_ftstatusuniqprocessguardoff_exitstatus(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_guardoff,
            sub_ftstatusuniqprocessguardoff_initstatus,
            status_guardoff_common,
            sub_guard_off_uniq,
            sub_status_guard_off_main_common_cancel,
            sub_status_guard_off_main_common_control,
            status_end_guardoff,
            sub_ftstatusuniqprocessguardoff_exitstatus
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}