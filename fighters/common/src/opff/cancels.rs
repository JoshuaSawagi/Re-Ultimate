use utils::{
    *,
    ext::*,
    consts::*
};

use utils::consts::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;

unsafe fn jump_cancel_grab(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, fighter_kind: i32) {
    // Link can JC grab, except while a bomb is out
    let link_has_bomb = fighter_kind == *FIGHTER_KIND_LINK
    && ArticleModule::is_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB);

    if !link_has_bomb {
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if boma.is_cat_flag(Cat1::Catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
            }
        }
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    jump_cancel_grab(boma, cat[0], status_kind, fighter_kind);
}
