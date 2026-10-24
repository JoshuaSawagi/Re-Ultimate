/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;
use globals::*;

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_jumpsquat_main,
            status_jumpsquat_common,
            status_jumpsquat,
            status_end_jumpsquat,
            uniq_process_JumpSquat_exec_status,
            uniq_process_JumpSquat_exec_status_param,
            sub_status_jumpsquat_check_stick_lr_update,
            sub_jump_squat_uniq_check_sub,
            sub_jump_squat_uniq_check_sub_mini_attack,
            
            
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

//#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon21status_JumpSquat_MainEv")]
//Status Jumpsquat Main, enables Wavedash out of Jumpsquat
#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe fn status_jumpsquat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
   //println!("main");
    // Check if a character (like greninja) has a custom subroutine for status checks
    let should_end = if fighter.global_table[CUSTOM_ROUTINE].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[CUSTOM_ROUTINE].get_ptr() as _;
        if !custom_routine.is_null() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine);
            callable(fighter).get_bool()
        } else {
            false
        }
    } else { false };
    if should_end {
        return L2CValue::I32(1);
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_JUMP),
            L2CValue::Bool(false)
        );
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL)
    && fighter.is_situation(*SITUATION_KIND_AIR)  {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
            L2CValue::Bool(false)
        );
        return 0.into();
    }
    /*if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_CATCH),
            L2CValue::Bool(true)
        );
        return 0.into();
    }*/
    if !fighter.sub_transition_group_check_ground_item().get_bool() {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)
            && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0
            && fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_SPECIAL_HI),
                L2CValue::Bool(true)
            );
        } else if !fighter.sub_transition_specialflag_hoist().get_bool() {
            // let cat2 = fighter.global_table[CMD_CAT2].get_i32();
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START)
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                if fighter.global_table[0x58].get_bool() != false && {
                    let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x58].get_ptr());
                    callable(fighter).get_bool()
                } {
                    return L2CValue::I32(0);
                }
                // if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_ATTACK_DASH_ATTACK_HI4 != 0 // original
                if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 // check if there is a valid stick flick using the original flag
                    && fighter.is_situation(*SITUATION_KIND_GROUND) {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_HI4_START),
                        L2CValue::Bool(true)
                    );
                }
            }
        }
    }
    0.into()
}

//#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon23status_JumpSquat_commonEN3lib8L2CValueE")]
#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_common)]
unsafe extern "C" fn status_jumpsquat_common(fighter: &mut L2CFighterCommon, lr_update: L2CValue) {
    let is_button_jump = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE) == 0
        || fighter.global_table[FLICK_Y_DIR].get_i32() <= 0;
    //InputModule::set_persist_lifetime(fighter.battle_object, 10);
    //InputModule::enable_persist(fighter.battle_object);
    if is_button_jump {
        //println!("button jump");
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_BUTTON);
        // check if we are doing double button shorthop input
        if ControlModule::is_jump_mini_button(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    // I think this int might be referring to how many frames we check for tap jump?
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    // `lr_update` comes from a dif subroutine
    if lr_update.get_bool() {
        //VarModule::on_flag(fighter.battle_object, vars::common::status::CSTICK_IRAR);
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    // Commented out so we can keep our current stick flick.
    // ControlModule::reset_flick_y(fighter.module_accessor);
    // ControlModule::reset_flick_sub_y(fighter.module_accessor);
    // fighter.global_table[FLICK_Y].assign(&0xFE.into());

    // not a conditional enable, so it's not in potential_enables
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    let potential_enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND
    ];
    for x in potential_enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ABNORMAL_MINIJUMP_SLOWWALK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    }
    // if we are doing a buffered aerial we want to disable all of the other transitions
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        for x in potential_enables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    }
    // same as above, but without the skip stuff
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
        for x in potential_enables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
    }
    
    // if you are jumping oos, we do not want to trigger jc grab. This avoids getting grabs when buffering an aerial oos.

    if fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF]) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    }

    VarModule::off_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
    sub_air_reset_escape_air_snap(fighter);
}

#[skyline::hook(replace = L2CFighterCommon_uniq_process_JumpSquat_exec_status)]
unsafe fn uniq_process_JumpSquat_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    uniq_process_JumpSquat_exec_status_param(fighter, L2CValue::Ptr(0 as _));
    0.into()
}


// The main exec block, for some reason it's not found in the exec status
#[skyline::hook(replace = L2CFighterCommon_uniq_process_JumpSquat_exec_status_param)]
unsafe extern "C" fn uniq_process_JumpSquat_exec_status_param(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    let should_check = if fighter.global_table[CUSTOM_ROUTINE].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[CUSTOM_ROUTINE].get_ptr() as _;
        if !custom_routine.is_null() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine);
            callable(fighter);
            true
        } else {
            true
        }
    } else { true };
    if should_check {
        fighter.sub_jump_squat_uniq_check_sub(L2CValue::I32(*FIGHTER_STATUS_JUMP_FLAG_BUTTON));
        fighter.sub_jump_squat_uniq_check_sub_mini_attack();
    }

    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let update_rate = MotionModule::update_rate(fighter.module_accessor);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0 {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
        && !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    }
    else {
        let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(motion_kind)) as u32 as f32;
        if end_frame <= (frame + update_rate) {
            StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
            fighter.global_table[PREV_SITUATION_KIND] = fighter.global_table[SITUATION_KIND].clone();
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT) {
                // check if we are doing directional airdodge
                let stick = app::sv_math::vec2_length(fighter.global_table[STICK_X].get_f32(), fighter.global_table[STICK_Y].get_f32());
                if stick >= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_stick")) {
                    VarModule::on_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
                    // change kinetic/ground properties for wavedash
                    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
                } else {
                    VarModule::off_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
                    // change kinetic properties for rising nairdodge
                    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
                }
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            } else {
                // change kinetic/ground properties for jump
                VarModule::off_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_SQUAT, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
            }
        }
        else {
            //println!("js_vel: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
            VarModule::set_float(fighter.battle_object, vars::common::instance::JUMPSQUAT_VELOCITY, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
            VarModule::set_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
            VarModule::set_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM_SPECIALS, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
        }
    }
}

// subroutine for checking for aerial macro
//#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon29sub_jump_squat_uniq_check_subEN3lib8L2CValueE")]
#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub)]
unsafe fn sub_jump_squat_uniq_check_sub(fighter: &mut L2CFighterCommon, flag: L2CValue) {
    //VarModule::inc_int(fighter.battle_object, vars::common::instance::JUMP_SQUAT_FRAME);

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) { return; }
    // flag is basically always going to be the jump button flag
    // checks if we are pressing a jump **button**
    if WorkModule::is_flag(fighter.module_accessor, flag.get_i32()){
        let mot = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(mot));
        //println!("button jump 2");
        // checks if we have released or if we are pressing two jump buttons, if so, reserve shorthop
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            || ControlModule::is_jump_mini_button(fighter.module_accessor) {
                //println!("button shorthop");
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
        // prevents cstick drift if inputting cstick on last 2 frames of jumpsquat
        /*if frame >= (end_frame - 1.0) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE) {
                //println!("2nd frame override");
                VarModule::on_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE_SECOND);
                ControlModule::reset_main_stick_x(fighter.module_accessor);
            }
            if frame >= (end_frame - 1.0) && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                //println!("override");
                VarModule::on_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE);
                ControlModule::reset_main_stick_x(fighter.module_accessor);
            }
        }*/
    } else {
        // if we are here, it means that we are using tap jump
        //VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_TAP_JUMP);

        // remove buffered aerial cstick drift
        /*if fighter.is_button_on(Buttons::CStickOverride) {
            ControlModule::reset_main_stick_x(fighter.module_accessor);
        }*/

        // compare the value of the left stick with the threshold for stick jumping
        if fighter.left_stick_y() < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y")) {
            // used to buffer specials and make sure that we aren't detecting when c stick is off
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
            && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            // uncomment for short hop aerial only
            // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
}

//#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
//symbol = "_ZN7lua2cpp16L2CFighterCommon16status_JumpSquatEv")]
#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat)]
unsafe fn status_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr_update = fighter.sub_status_JumpSquat_check_stick_lr_update();
    fighter.status_JumpSquat_common(lr_update);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_jumpsquat_main as *const () as _))
}

//#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon42sub_status_JumpSquat_check_stick_lr_updateEv")]
#[skyline::hook(replace = L2CFighterCommon_sub_status_JumpSquat_check_stick_lr_update)]
unsafe fn sub_status_jumpsquat_check_stick_lr_update(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    /*only allow jumpsquat to flip you around if your previous status was Dash and 
    your directional input was caused by cstick (cstick input 2 frames within jumpsquat*/
    
    // allows for cstick IRAR
    L2CValue::Bool(prev_status == *FIGHTER_STATUS_KIND_DASH && fighter.is_button_on(Buttons::CStickOverride))
}
    

//Status End Jumpsquat, clears flags
//#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
   // symbol = "_ZN7lua2cpp16L2CFighterCommon20status_end_JumpSquatEv" )]
#[skyline::hook(replace = L2CFighterCommon_status_end_JumpSquat)]
unsafe fn status_end_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    InputModule::disable_persist(fighter.battle_object);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE_SECOND);
    VarModule::set_int(fighter.battle_object, vars::common::instance::JUMP_SQUAT_FRAME, 0);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_TAP_JUMP);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_ATTACK_CANCEL);
    0.into()
}

//#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon41sub_jump_squat_uniq_check_sub_mini_attackEv")]
#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub_mini_attack)]
unsafe fn sub_jump_squat_uniq_check_sub_mini_attack(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) { return; }
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON)
            && current_frame > 0.0 {
            FighterControlModuleImpl::reserve_on_attack_button(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        }
    } else {
        if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
                return;
            }
        } else if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
            return;
        }
        let unables = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW
        ];
        for x in unables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
}

pub unsafe fn sub_air_reset_escape_air_snap(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::common::instance::AIR_ESCAPE_MAGNET_FRAME, 6);
    // println!("magnet_frame {}", magnet_frame);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET);
}