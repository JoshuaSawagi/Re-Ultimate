use super::*;
use crate::consts::globals::*;
use crate::consts::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyMotionResetType {
    GroundTransLoop = 0x0,
    GroundTransLoopGekikara,
    GroundTrans,
    GroundTransIgnoreNorm,
    AirTrans,
    AirTransAngle,
    AirTransY,
    AirTransAngleSuperJumpPunch,
    AirTrans2nd,
    CliffTransIntp,
    CliffTrans,
    CliffTransGround,
    LadderMove,
    LadderTrans,
}

impl EnergyMotionResetType {
    pub fn is_ground(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, GroundTransLoop | GroundTransLoopGekikara | GroundTrans | GroundTransIgnoreNorm)
    }

    pub fn is_air(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, AirTrans | AirTransAngle | AirTransY | AirTransAngleSuperJumpPunch | AirTrans2nd)
    }

    pub fn is_cliff(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, CliffTransIntp | CliffTrans | CliffTransGround)
    }

    pub fn is_ladder(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, LadderMove | LadderTrans)
    }

    pub fn is_2nd(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, AirTrans2nd)
    }
}

#[repr(C)]
pub struct FighterKineticEnergyMotion {
    parent: super::energy::KineticEnergy,
    pub lr: f32,
    pub angle: f32,
    pub angle_whole: f32,
    pub angle_intp_end: f32,
    pub angle_intp_frames_remaining: i32,
    pub speed_mul: f32,
    pub prev_speed: PaddedVec2,
    pub speed_mul_2nd: PaddedVec2,
    pub update_flag: bool,
    // ...
}

impl Deref for FighterKineticEnergyMotion {
    type Target = super::energy::KineticEnergy;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterKineticEnergyMotion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl FighterKineticEnergyMotion {
    pub fn update_trans_move_speed_2nd(boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let motion_module_vtable = *(motion_module as *const *const u64);
            let function: extern "C" fn(u64) = std::mem::transmute(*motion_module_vtable.add(0x220 / 0x8));
            function(motion_module);
        }
    }

    pub fn is_motion_2nd_updating_energy(boma: &mut BattleObjectModuleAccessor) -> bool {
        unsafe {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let motion_module_vtable = *(motion_module as *const *const u64);
            let function: extern "C" fn(u64) -> bool = std::mem::transmute(*motion_module_vtable.add(0x1f0 / 0x8));
            function(motion_module)
        }
    }

    pub fn is_main_motion_updating_energy(boma: &mut BattleObjectModuleAccessor) -> bool {
        unsafe {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let motion_module_vtable = *(motion_module as *const *const u64);
            let function: extern "C" fn(u64) -> bool = std::mem::transmute(*motion_module_vtable.add(0x1e8 / 0x8));
            function(motion_module)
        }
    }

    pub fn trans_move_speed_correct(boma: &mut BattleObjectModuleAccessor) -> Vector3f {
        unsafe {
            let func: extern "C" fn(&mut BattleObjectModuleAccessor) -> smash_rs::cpp::simd::Vector3 = std::mem::transmute(MotionModule::trans_move_speed as *const ());
            let vec = func(boma);
            Vector3f {
                x: vec.x(),
                y: vec.y(),
                z: vec.z(),
            }
        }
    }

    pub fn trans_move_speed_2nd_correct(boma: &mut BattleObjectModuleAccessor) -> Vector3f {
        unsafe {
            let func: extern "C" fn(&mut BattleObjectModuleAccessor) -> smash_rs::cpp::simd::Vector3 = std::mem::transmute(MotionModule::trans_move_speed_2nd as *const ());
            let vec = func(boma);
            Vector3f {
                x: vec.x(),
                y: vec.y(),
                z: vec.z(),
            }
        }
    }

    pub fn set_values_and_process(&mut self, accel: PaddedVec2, max_speed: PaddedVec2, speed: PaddedVec2, boma: &mut BattleObjectModuleAccessor) {
        self.accel = accel;
        self.speed_max = max_speed;
        self.process(boma);
        self.active_flag = true;
        self.prev_speed = speed;
    }

    pub fn get_translation_by_reset_type(boma: &mut BattleObjectModuleAccessor, reset_type: EnergyMotionResetType) -> PaddedVec2 {
        let translation = unsafe {
            if reset_type.is_2nd() {
                Self::update_trans_move_speed_2nd(boma);
                Self::trans_move_speed_2nd_correct(boma)
            } else {
                MotionModule::update_trans_move_speed(boma);
                Self::trans_move_speed_correct(boma)
            }
        };

        PaddedVec2::new(translation.z, translation.y)
    }

    pub fn is_motion_updating_energy(boma: &mut BattleObjectModuleAccessor, reset_type: EnergyMotionResetType) -> bool {
        if reset_type.is_2nd() {
            Self::is_motion_2nd_updating_energy(boma)
        } else {
            Self::is_main_motion_updating_energy(boma)
        }
    }
}

#[skyline::from_offset(0x6941e0)]
extern "C" fn handle_cliff(boma: &mut BattleObjectModuleAccessor, vec: &Vector4f) -> smash_rs::cpp::simd::Vector4;

#[skyline::hook(offset = 0x6d5cb0)]
unsafe fn motion_update(energy: &mut FighterKineticEnergyMotion, boma: &mut BattleObjectModuleAccessor) {
    use EnergyMotionResetType::*;
    let reset_type = std::mem::transmute(energy.energy_reset_type);

    energy.active_flag = true;
    if !FighterKineticEnergyMotion::is_motion_updating_energy(boma, reset_type) {
        let backup_brake = energy.speed_brake;

        if reset_type == LadderMove {
            energy.set_values_and_process(PaddedVec2::new(-energy.speed.x, -energy.speed.y), PaddedVec2::zeros(), PaddedVec2::zeros(), boma);
            return;
        }

        if reset_type.is_ground() {
            energy.speed_limit = PaddedVec2::new(WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ground_speed_limit")), 0.0);
        }

        energy.set_values_and_process(PaddedVec2::zeros(), PaddedVec2::zeros(), PaddedVec2::zeros(), boma);

        energy.speed_brake = backup_brake;

        return;
    }

    let mut is_stop_added = false;

    if !energy.update_flag && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3]) {
        let mut stop_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        let prev_speed = KineticModule::get_sum_speed3f(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let reset_speed_2f = Vector2f {
            x: prev_speed.x,
            y: prev_speed.y,
        };
        let reset_speed_3f = Vector3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_GROUND, &reset_speed_2f, &reset_speed_3f, boma);
        lua_bind::KineticEnergy::enable(stop_energy);

        is_stop_added = true;
    }

    let mut move_speed = FighterKineticEnergyMotion::get_translation_by_reset_type(boma, reset_type);

    if energy.angle_intp_frames_remaining >= 1 {
        energy.angle_whole += (energy.angle_intp_end - energy.angle_whole) / energy.angle_intp_frames_remaining as f32;
        energy.angle_intp_frames_remaining -= 1;
    };

    if energy.angle_whole != 0.0 {
        move_speed.x = move_speed.x * energy.angle_whole.cos() - move_speed.y * energy.angle_whole.sin();
        move_speed.y = move_speed.y * energy.angle_whole.cos() + move_speed.x * energy.angle_whole.sin();
    }

    let lr = if matches!(reset_type, GroundTransLoop | GroundTransLoopGekikara | AirTransAngle | AirTransAngleSuperJumpPunch) {
        PostureModule::lr(boma)
    } else {
        energy.lr
    };

    move_speed.x *= lr * energy.speed_mul * energy.speed_mul_2nd.x;
    move_speed.y *= energy.speed_mul * energy.speed_mul_2nd.y;

    energy.active_flag = false;

    if boma.status_frame() == 0 {
        move_speed.x = if is_stop_added {
            0.0
        } else {
            energy.prev_speed.x
        };
    }

    let speed = match reset_type {
        GroundTransLoop => {
            energy.active_flag = true;
            energy.speed_limit = PaddedVec2::new(-1.0, 0.0);
            move_speed
        }

        GroundTransIgnoreNorm => {
            energy.speed_limit = PaddedVec2::new(-1.0, 0.0);
            move_speed
        }

        GroundTrans => {
            energy.speed_limit = PaddedVec2::new(-1.0, 0.0);
            energy::KineticEnergy::adjust_speed_for_ground_normal(&move_speed, boma)
        }

        GroundTransLoopGekikara => {
            energy.active_flag = true;
            energy.speed_limit = PaddedVec2::new(-1.0, 0.0);
            let some_rate = WorkModule::get_float(boma, 0x1000009);
            let motion_rate = MotionModule::rate(boma);
            if some_rate != 0.0 && motion_rate / some_rate != 0.0 {
                PaddedVec2::new(move_speed.x * some_rate / motion_rate, move_speed.y * some_rate / motion_rate)
            } else {
                PaddedVec2::zeros()
            }
        }

        AirTrans | AirTrans2nd => move_speed,

        AirTransAngle => PaddedVec2::new(move_speed.x * energy.angle.cos() - move_speed.y * energy.angle.sin(), move_speed.y * energy.angle.cos() + move_speed.x * energy.angle.sin()),

        AirTransY => PaddedVec2::new(0.0, move_speed.y),

        AirTransAngleSuperJumpPunch => {
            let stick_x = ControlModule::get_stick_x(boma);
            let dir = WorkModule::get_float(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
            let angle = if stick_x.abs() <= dir {
                energy.angle
            } else {
                let interp = (stick_x.abs() - dir) / (1.0 - dir);
                let interp = interp * WorkModule::get_float(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
                let new_angle = if stick_x <= 0.0 {
                    interp.to_radians()
                } else {
                    -interp.to_radians()
                };
                if energy.angle.abs() < new_angle.abs() {
                    energy.angle = new_angle;
                }
                energy.angle
            };
            PaddedVec2::new(move_speed.x * angle.cos() - move_speed.y * energy.angle.sin(), move_speed.y * angle.cos() + move_speed.x * energy.angle.sin())
        }

        CliffTransIntp | CliffTrans | CliffTransGround => {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let motion_vtable = *(motion_module as *const *const u64);
            let some_func: extern "C" fn(u64) -> smash_rs::cpp::simd::Vector4 = std::mem::transmute(*motion_vtable.add(0x230 / 0x8));
            let vec = some_func(motion_module);
            let vec = Vector4f {
                x: vec.x(),
                y: vec.y(),
                z: vec.z(),
                w: vec.w(),
            };
            if reset_type == CliffTransGround {
                energy.active_flag = true;
            }
            let vec = handle_cliff(boma, &vec);
            if reset_type == CliffTransIntp {
                let frame = WorkModule::get_int(boma, 0x11000005);
                let interpolated = 1.0 / (frame + 1) as f32;
                PaddedVec2::new(vec.x() * interpolated, vec.y() * interpolated)
            } else {
                PaddedVec2::new(vec.x(), vec.y())
            }
        }

        LadderMove => {
            energy.active_flag = true;
            let stick_y = ControlModule::get_stick_y(boma);
            let speed_y = if 0.5 <= stick_y.abs() {
                if stick_y <= 0.0 {
                    -WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_speed_d_max")) * MotionModule::rate(boma)
                } else {
                    WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_speed_u_max")) * MotionModule::rate(boma)
                }
            } else {
                0.0
            };

            PaddedVec2::new(0.0, speed_y)
        }

        LadderTrans => {
            energy.active_flag = true;
            let ladder_end_y = WorkModule::get_float(boma, *FIGHTER_STATUS_LADDER_WORK_FLOAT_LADDER_END_Y);
            let ladder_end_start_y = WorkModule::get_float(boma, *FIGHTER_STATUS_LADDER_WORK_FLOAT_LADDER_END_START_Y);
            let mut vec = Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            MotionModule::trans_tra(boma, &mut vec, true, true);
            let speed_y = (ladder_end_y + vec.y) - ladder_end_start_y;
            WorkModule::add_float(boma, speed_y, *FIGHTER_STATUS_LADDER_WORK_FLOAT_LADDER_END_START_Y);
            PaddedVec2::new(0.0, speed_y)
        } // _ => {}
    };

    if reset_type.is_ground() && energy.update_flag && speed.x == 0.0 && energy.prev_speed.x == 0.0 {
        energy.set_values_and_process(PaddedVec2::zeros(), PaddedVec2::zeros(), speed, boma);
        return;
    }

    let speed_to_change_from = if energy.update_flag {
        energy.prev_speed
    } else {
        energy.update_flag = true;
        energy.speed
    };

    energy.set_values_and_process(PaddedVec2::new(speed.x - speed_to_change_from.x, speed.y - speed_to_change_from.y), PaddedVec2::new(-1.0, -1.0), speed, boma);
}

pub fn install() {
    skyline::install_hooks!(motion_update);
}
