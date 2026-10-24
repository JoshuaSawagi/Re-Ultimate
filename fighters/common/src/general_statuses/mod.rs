use super::*;
use globals::*;
use interpolation::Lerp;

pub static VECTOR_ZERO : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

macro_rules! interrupt {
    () => { return L2CValue::I32(1); };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}

mod guard_damage;
mod guard_off;
mod guard_on;
mod guard;
pub mod jump;
mod jumpsquat;
mod turn;

pub fn install() {
    guard_damage::install();
    guard_off::install();
    guard_on::install();
    guard::install();
    jump::install();
    jumpsquat::install();
    turn::install();
    

    //skyline::nro::add_hook(nro_hook);
}