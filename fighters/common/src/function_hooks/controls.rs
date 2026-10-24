use super::*;
use utils::ext::*;

#[repr(C)]
struct SomeControllerStruct {
    padding: [u8; 0x10],
    controller: &'static mut Controller
}


#[skyline::hook(offset = offsets::map_controls())]
unsafe fn map_controls_hook(
    mappings: *mut ControllerMapping,
    player_idx: i32,
    out: *mut MappedInputs,
    controller_struct: &mut SomeControllerStruct,
    arg: bool
) {
    let ret = original!()(mappings, player_idx, out, controller_struct, arg);
    let controller = &mut controller_struct.controller;

    // Check if the button combos are being pressed and then force Stock Share + AttackRaw/SpecialRaw depending on input

    if controller.current_buttons.l()
    && controller.current_buttons.r()
    && controller.current_buttons.a()
    && (controller.current_buttons.minus() || controller.current_buttons.plus())
    {
        controller.current_buttons.set_plus(false);
        controller.current_buttons.set_minus(false);
        controller.just_down.set_plus(false);
        controller.just_down.set_minus(false);

        if controller.current_buttons.y() {
            (*out).buttons = Buttons::StockShare | Buttons::AttackRaw;
        } else if controller.current_buttons.x() {
            (*out).buttons = Buttons::StockShare | Buttons::SpecialRaw;
        } else {
            controller.current_buttons.set_plus(true);
            controller.current_buttons.set_minus(true);
            controller.just_down.set_plus(true);
            controller.just_down.set_minus(true);
        }
    }
}

// These 2 hooks prevent buffered nair after inputting C-stick on first few frames of jumpsquat
// Both found in ControlModule::exec_command
#[skyline::hook(offset = 0x6be630)]
unsafe fn set_attack_air_stick_hook(control_module: u64, arg: u32) {
    // This check passes on the frame FighterControlModuleImpl::reserve_on_attack_button is called
    // Only happens during jumpsquat currently
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    if *((control_module + 0x645) as *const bool)
    && !VarModule::is_flag((*boma).object(), vars::common::instance::IS_ATTACK_CANCEL)
    && !VarModule::is_flag((*boma).object(), vars::common::status::CSTICK_IRAR) {
        return;
    }
    call_original!(control_module, arg);
}
#[skyline::hook(offset = 0x6bd6c4, inline)]
unsafe fn exec_command_reset_attack_air_kind_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[21].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    // For some reason, the game resets your attack_air_kind value every frame
    // even though it resets as soon as you perform an aerial attack
    // We don't want this to reset while in jumpsquat
    // to allow the game to use your initial C-stick input during jumpsquat for your attack_air_kind
    if !(*boma).is_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        ControlModule::reset_attack_air_kind(boma);
    }
}

pub fn install() {
    // Removes 10f C-stick lockout for tilt stick and special stick
    skyline::patching::Patch::in_text(0x17532ac).data(0x2A1F03FA);
    skyline::patching::Patch::in_text(0x17532b0).nop();
    skyline::patching::Patch::in_text(0x17532b4).nop();
    skyline::patching::Patch::in_text(0x17532b8).nop();

    // Prevents buffered C-stick aerials from triggering nair
    skyline::patching::Patch::in_text(0x6be664).data(0x52800040);

    // Prevents attack_air_kind from resetting every frame
    // Found in ControlModule::exec_command
    skyline::patching::Patch::in_text(0x6bd6c4).nop();

    skyline::install_hooks!(
        map_controls_hook,
        exec_command_reset_attack_air_kind_hook
       
    );
}