mod specials;
use specials::is_special;
use specials::is_anyplayer_final;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash::*;
use smash::app::{BattleObjectModuleAccessor, Fighter, FighterUtil, SituationKind, smashball};
use std::mem;
use smash::app::sv_module_access;
use smash::app::sv_battle_object;
use smash::phx::{Hash40, Vector4f};
use skyline::nn::ro::LookupSymbol;
use std::time::Duration;
use skyline::c_str;
use skyline::libc::c_int;
use smash::app::lua_bind::MotionModule::end_frame;
use smash::app::lua_bind::StatusModule::{prev_status_kind, status_kind};
use smash::app::smashball::is_training_mode;
use smash::app::sv_math::rand;
use smash::lua2cpp::L2CFighterCommon;
use smash::cpp::root::app::lua_bind::ArticleModule::change_motion;
use smash::lib::*;
use crate::ultimex::specials::*;
use crate::ultimex::specials::is_special_hi;


static mut FIGHTER_MANAGER_ADDR: usize = 0;
static mut AIRDODGE : [i32; 9] = [2; 9];
static mut POSX : [f32; 9] = [-1.0 ; 9];
static mut POSY : [f32; 9] = [-1.0 ; 9];
static mut OPPONENT_ID : [usize; 9] = [9; 9];
static mut AIRTAUNT_USED :[bool; 9] =[false;9];
static mut IS_LEFT :[bool;8] =[false;8];
static mut IS_RIGHT:[bool;8] =[false;8];
static mut TECH_FRAME: [i32; 9] = [0; 9];
static mut IS_ATK_SMASH_HIT:bool = false;
static mut SMASH_HIT_ENTRY_ID:i32 = 0;
static mut TIMES_ATTACKED:[i32;9] = [0;9];
static mut HIT_FRAME_COUNTER:[f32;8] = [0.0;8];
static mut CTR_HIT_INVIN:[f32;8] = [0.0;8];
static mut IS_CTR_HIT:[bool;8] =[false;8];
static mut AIR_TIME_COUNTER:[f32;8] = [0.0;8];
static mut BACK_HOLD_COUNTER:[f32;8] = [0.0;8];
static mut CAN_TIMER_COUNT:[bool;8] =[false;8];
static mut HIT_FRAME:[f32;8] =[0.0;8];
static mut DASH_START_X:[f32;8] =[0.0;8];
static mut IS_ATKDASH_RIGHT:[bool;8] =[false;8];
static mut IS_ATKDASH_LEFT:[bool;8] =[false;8];
static mut IS_TURNING:[bool;8] =[false;8];
static mut IS_USED_PUMMEL:[bool;8] =[false;8];
static mut IS_BACK_SMASH:[bool;8] =[false;8];
static mut IS_BACK_TILT:[bool;8] =[false;8];
static mut IS_USED_DATK_SUPER_ARMOR:[bool;8] =[false;8];
static mut IS_USED_SMASH_SUPER_ARMOR:[bool;8] =[false;8];
static mut IS_FLIGHT:[bool;8] = [false; 8];
static mut NUM_AERIALS:[i32;8] = [0; 8];
static mut IS_TURBO:[bool;8] = [false; 8];
static mut CAN_FLY:[bool;8] = [false; 8];
static mut JUMP_HOLD_TIMER:[f32;8] = [0.0; 8];
static mut IS_USED_AERIAL_2:[bool;8] = [false; 8];

#[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ENTRY_ID = get_entry_id(module_accessor);
    if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER &&
        [*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&StatusModule::status_kind(module_accessor))
        && !(fighter_kind == *FIGHTER_KIND_SZEROSUIT && StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW4){
        let mut l2c_agent = L2CAgent::new(lua_state);
        let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for i in 0..36 {
            if i == 4 && IS_BACK_SMASH[ENTRY_ID]{
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(270.0.into()));
            }
            else if i == 5 && !hitbox_params[6].get_f32().is_normal() && hitbox_params[i].get_f32() > 35.0{
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(80.0.into()));
            }
            else if i == 6 && !hitbox_params[i].get_f32().is_normal() {
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(70.0.into()));
            }
            else if i == 7 && !hitbox_params[6].get_f32().is_normal() {
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(100.0.into()));
            }
            else {
                l2c_agent.push_lua_stack(&mut hitbox_params[i]);
            }
        }
    }
    if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER &&
        [*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&StatusModule::status_kind(module_accessor)) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for i in 0..36 {
            if i == 18 && IS_BACK_TILT[ENTRY_ID]{
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*ATTACK_LR_CHECK_B as u64));
            }
            else{
                l2c_agent.push_lua_stack(&mut hitbox_params[i]);
            }
        }
    }
    if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER &&
        [hash40("attack_air_b")].contains(&MotionModule::motion_kind(module_accessor)) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for i in 0..36 {
            if i == 18{
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*ATTACK_LR_CHECK_F as u64));
            }
            else{
                l2c_agent.push_lua_stack(&mut hitbox_params[i]);
            }
        }
    }
    /*
    if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER &&
        [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&StatusModule::status_kind(module_accessor)) && FighterUtil::is_hp_mode(module_accessor) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for i in 0..36 {
            if i == 5 && !hitbox_params[6].get_f32().is_normal() && hitbox_params[i].get_f32() > 35.0{
                l2c_agent.push_lua_stack(&mut L2CValue::new_num((hitbox_params[i].get_f32() * 0.75).into()));
            }
            else if i == 6 && !hitbox_params[i].get_f32().is_normal() {
                l2c_agent.push_lua_stack(&mut L2CValue::new_num((hitbox_params[i].get_f32() * 0.75).into()));
            }
            else if i == 7 && !hitbox_params[6].get_f32().is_normal() {
                l2c_agent.push_lua_stack(&mut L2CValue::new_num((hitbox_params[i].get_f32() * 0.75 ).into()));
            }
            else {
                l2c_agent.push_lua_stack(&mut hitbox_params[i]);
            }
        }
    }
     */
    original!()(lua_state);
}

pub unsafe fn is_atk_air_input(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    [*FIGHTER_COMMAND_ATTACK_AIR_KIND_B, *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,
        *FIGHTER_COMMAND_ATTACK_AIR_KIND_N, *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,
        *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW].contains(&ControlModule::get_attack_air_kind(module_accessor))
}

pub unsafe fn is_damage_check(module_accessor : *mut BattleObjectModuleAccessor, is_prev : bool) -> bool {
    let status : i32;
    if is_prev {
        status = StatusModule::prev_status_kind(module_accessor, 0);
    }
    else {
        status = StatusModule::status_kind(module_accessor);
    }
    if FighterStopModuleImpl::is_damage_stop(module_accessor) || CaptureModule::is_capture(module_accessor)
        || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
        || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
        || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
        || [
        *FIGHTER_STATUS_KIND_AIR_LASSO,
        *FIGHTER_STATUS_KIND_BIND,
        *FIGHTER_STATUS_KIND_BURY,
        *FIGHTER_STATUS_KIND_BURY_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_BEETLE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_DRIVER,
        *FIGHTER_STATUS_KIND_CAPTURE_ITEM,
        *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
        *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND,
        *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_YOSHI,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY,
        *FIGHTER_STATUS_KIND_CATCHED_REFLET,
        *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED,
        *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN,
        *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY,
        *FIGHTER_STATUS_KIND_CLUNG_DIDDY,
        *FIGHTER_STATUS_KIND_CLUNG_GANON,
        *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY,
        *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG_END,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG_START,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_DOWN,
        *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
        *FIGHTER_STATUS_KIND_DOWN_EAT,
        *FIGHTER_STATUS_KIND_DOWN_SPOT,
        *FIGHTER_STATUS_KIND_DOWN_STAND,
        *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        *FIGHTER_STATUS_KIND_DOWN_WAIT,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_STATUS_KIND_FURAFURA,
        *FIGHTER_STATUS_KIND_FURAFURA_END,
        *FIGHTER_STATUS_KIND_FURAFURA_STAND,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
        *FIGHTER_STATUS_KIND_ICE,
        *FIGHTER_STATUS_KIND_KOOPA_DIVED,
        *FIGHTER_STATUS_KIND_LAY_DOWN,
        *FIGHTER_STATUS_KIND_MEWTWO_THROWN,
        *FIGHTER_STATUS_KIND_MISS_FOOT,
        *FIGHTER_STATUS_KIND_PASSIVE,
        *FIGHTER_STATUS_KIND_PASSIVE_CEIL,
        *FIGHTER_STATUS_KIND_PASSIVE_FB,
        *FIGHTER_STATUS_KIND_PASSIVE_WALL,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN,
        *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL,
        *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY,
        *FIGHTER_STATUS_KIND_SLEEP,
        *FIGHTER_STATUS_KIND_SLIP,
        *FIGHTER_STATUS_KIND_SLIP_DAMAGE,
        *FIGHTER_STATUS_KIND_SLIP_WAIT,
        *FIGHTER_STATUS_KIND_SLIP_STAND,
        *FIGHTER_STATUS_KIND_SLIP_STAND_B,
        *FIGHTER_STATUS_KIND_SLIP_STAND_F,
        *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK,
        *FIGHTER_STATUS_KIND_STABBED_DAMAGE,
        *FIGHTER_STATUS_KIND_STABBED_RIDLEY,
        *FIGHTER_STATUS_KIND_SWALLOWED,
        *FIGHTER_STATUS_KIND_THROWN,
    ].contains(&status) {
        true
    }
    else {
        false
    }
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_FALL_SPECIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn fall_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(special_fall as *const () as _))
}

unsafe extern "C" fn special_fall(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
    L2CValue::I32(0)
}

pub unsafe fn get_fighter_manager() -> *mut smash::app::FighterManager{
    *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager)
}

/*
#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn catch_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(&mut *fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_attack"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack as *const () as _))
}


unsafe extern "C" fn catch_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(&mut *fighter.module_accessor);
    if IS_USED_PUMMEL[ENTRY_ID]{
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
    }
    return L2CValue::I32(1)
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn catch_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(&mut *fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    IS_USED_PUMMEL[ENTRY_ID] = true;
    call_original!(fighter)
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn catch_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(&mut *fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(catch_wait_main as *const () as _))
}
unsafe extern "C" fn catch_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(&mut *fighter.module_accessor);
    return L2CValue::I32(0)
}
*/
mod FighterCtrlModuleImpl2{
    use smash::app::BattleObjectModuleAccessor;
    extern "C"{
        #[link_name = "\u{1}_ZN3app8lua_bind45FighterControlModuleImpl__delete_command_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn delete_command(module_accessor: &mut BattleObjectModuleAccessor, arg2: i32, arg3: u32);
    }
}
#[skyline::hook(replace=FighterCtrlModuleImpl2::delete_command)]
pub unsafe fn delete_command_hook(module_accessor: &mut BattleObjectModuleAccessor, arg2: i32, arg3: u32){
    /*
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_DEMON && status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP{
        return;
    }
    else{
        original!()(module_accessor, arg2, arg3);
    }

     */
}

#[smashline::status_script(agent = "demon", status = FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn kazuya_crouch_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    enable_dash_force(&mut *fighter.module_accessor);
    original!(fighter)
}

/*
#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_TURN_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn turn_dash_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(back_dash as *const () as _))
}
 */

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn dash_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    IS_DASH_BACK[ENTRY_ID] = false;
    call_original!(fighter)
}


#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn escape_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(&mut *fighter.module_accessor);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && IS_FLIGHT[ENTRY_ID]{
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), false.into());
    }
    call_original!(fighter)
}
pub unsafe fn air_taunt(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
    if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR{
        let stop_rise  = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 1.0 };
        if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI){
            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            MotionModule::change_motion(module_accessor, smash::phx::Hash40{hash: hash40("appeal_hi_l")}, 0.0, 1.0, false, 0.0, false, false);
            CancelModule::enable_cancel(module_accessor);
            if AIRTAUNT_USED[ENTRY_ID] == false{
                KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::mul_accel(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                AIRTAUNT_USED[ENTRY_ID] = true;
            }
        }else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW){
            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            MotionModule::change_motion(module_accessor, smash::phx::Hash40{hash: hash40("appeal_lw_l")}, 0.0, 1.0, false, 0.0, false, false);
            CancelModule::enable_cancel(module_accessor);
            if AIRTAUNT_USED[ENTRY_ID] == false{
                KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::mul_accel(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                AIRTAUNT_USED[ENTRY_ID] = true;
            }
        }else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L){
            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            MotionModule::change_motion(module_accessor, smash::phx::Hash40{hash: hash40("appeal_s_l")}, 0.0, 1.0, false, 0.0, false, false);
            CancelModule::enable_cancel(module_accessor);
            if AIRTAUNT_USED[ENTRY_ID] == false{
                KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::mul_accel(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                AIRTAUNT_USED[ENTRY_ID] = true;
            }
        }else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R){
            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            MotionModule::change_motion(module_accessor, smash::phx::Hash40{hash: hash40("appeal_s_r")}, 0.0, 1.0, false, 0.0, false, false);
            CancelModule::enable_cancel(module_accessor);
            if AIRTAUNT_USED[ENTRY_ID] == false{
                KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::mul_accel(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                AIRTAUNT_USED[ENTRY_ID] = true;
            }
        }


    }

    if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_GROUND{
        AIRTAUNT_USED[ENTRY_ID] = false;
    }
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL{
        CancelModule::enable_cancel(module_accessor);
    }
}
pub fn get_module_accessor(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
    unsafe {
        &mut *smash::app::sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(entry_id))
    }
}/*
static mut IS_ALLOW_ESCAPE:[bool;8] = [false;8];
static mut IS_ATK_S4:[bool;8] = [false;8];
static mut LAND_ATK_FLAG:[bool;8] = [false;8];
*/
unsafe fn change_status(module_accessor: *mut BattleObjectModuleAccessor, status: i32){
    StatusModule::change_status_request_from_script(module_accessor, status, true);
}

#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::is_generatable)]
pub unsafe fn is_generatable_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, article: i32) -> bool {
    return true;
}




/*pub fn cancel(module_accessor: &mut smash::app::BattleObjectModuleAccessor){
  unsafe{
  CancelModule::enable_cancel(module_accessor);
  //disable for now might revert
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
  WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
  }
}*/

#[skyline::hook(replace = smash::app::sv_animcmd::REVERSE_LR)]
pub unsafe fn REVERSE_LR_HOOK(arg1: u64){

}

static mut LAST_LR:[bool;8] = [false;8];
static mut CAN_TURN:[bool;8] = [false;8];
static mut BACKWARDS_LR:[f32;8] = [0.0;8];
static mut CORRECT_LR:[f32;8] = [0.0;8];

pub unsafe fn is_cloud_ganon_dsmash(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    [*FIGHTER_KIND_CLOUD, *FIGHTER_KIND_GANON].contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
}

pub unsafe fn is_ganon_captain_reverse_punch(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    [*FIGHTER_KIND_GANON, *FIGHTER_KIND_CAPTAIN].contains(&fighter_kind) &&
        [*FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind)
}

pub unsafe fn is_sword_dance(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    [*FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_ROY, *FIGHTER_KIND_CHROM].contains(&fighter_kind) &&
        [*FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2,
            *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3,
            *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4].contains(&status_kind)
}
pub unsafe fn get_entry_id(module_accessor: &mut BattleObjectModuleAccessor) -> usize{
    WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

pub unsafe fn can_autoturn(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    let ENTRY_ID = get_entry_id(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let FIGHTER_MANAGER = get_fighter_manager();
    let status_kind = StatusModule::status_kind(module_accessor);

    let is_allow_turn = MotionModule::motion_kind(module_accessor) != hash40("attack_air_b") && fighter_kind != *FIGHTER_KIND_NANA &&
        status_kind != *FIGHTER_STATUS_KIND_THROW && !is_special_hi(module_accessor, false) && !is_special_s(module_accessor, false) &&
         // ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100].contains(&status_kind) &&
        // ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) &&
        ![*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F].contains(&status_kind) &&
        ![*FIGHTER_STATUS_KIND_ITEM_THROW].contains(&status_kind) &&
        !is_cloud_ganon_dsmash(module_accessor) && !CaptureModule::is_capture(module_accessor) && !FighterManager::is_result_mode(FIGHTER_MANAGER) &&
        !is_ganon_captain_reverse_punch(module_accessor) &&
        !GroundModule::is_attach_cliff(module_accessor) &&
        ![*FIGHTER_DONKEY_STATUS_KIND_SHOULDER_END,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT_B,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_PASS,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK].contains(&status_kind) &&
        ![*FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN,
            *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, *FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK].contains(&status_kind);
    is_allow_turn
}



pub unsafe fn auto_turnaround(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    let FIGHTER_MANAGER = get_fighter_manager();
    let status_kind = StatusModule::status_kind(module_accessor);
        if ENTRY_ID == 0 || ENTRY_ID == 1 {
            POSX[ENTRY_ID] = PostureModule::pos_x(module_accessor);
            POSY[ENTRY_ID] = PostureModule::pos_y(module_accessor);
        }
        if ENTRY_ID == 0 && can_autoturn(module_accessor){
            if POSX[ENTRY_ID] < POSX[1] {
                BACKWARDS_LR[ENTRY_ID] = -1.0;
                CORRECT_LR[ENTRY_ID] = 1.0;
                PostureModule::set_lr(module_accessor, 1.0);
                PostureModule::update_rot_y_lr(module_accessor);
            }
            else {
                BACKWARDS_LR[ENTRY_ID] = 1.0;
                CORRECT_LR[ENTRY_ID] = -1.0;
                PostureModule::set_lr(module_accessor, -1.0);
                PostureModule::update_rot_y_lr(module_accessor);
            }
        }
        if ENTRY_ID == 1 &&  can_autoturn(module_accessor){
            if POSX[ENTRY_ID] < POSX[0] {
                BACKWARDS_LR[ENTRY_ID] = -1.0;
                CORRECT_LR[ENTRY_ID] = 1.0;
                PostureModule::set_lr(module_accessor, 1.0);
                PostureModule::update_rot_y_lr(module_accessor);

            }
            else {
                BACKWARDS_LR[ENTRY_ID] = 1.0;
                CORRECT_LR[ENTRY_ID] = -1.0;
                PostureModule::set_lr(module_accessor, -1.0);
                PostureModule::update_rot_y_lr(module_accessor);
            }
        }
    if MotionModule::motion_kind(module_accessor) == hash40("attack_air_b"){
        PostureModule::set_lr(module_accessor, BACKWARDS_LR[ENTRY_ID]);
        PostureModule::update_rot_y_lr(module_accessor);
    }
    disable_turn(module_accessor);
}

pub unsafe fn throw_cancels(module_accessor: &mut BattleObjectModuleAccessor){
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_THROW{
        if MotionModule::frame(module_accessor) > 5.0{
            CancelModule::enable_cancel(module_accessor);
            disable_jab(module_accessor);
            enable_dash_force(module_accessor);
            /*
            disable_turn(module_accessor);
        disable_run(module_accessor);
        disable_walk(module_accessor);
        disable_dash(module_accessor);
        //disable_tilts(module_accessor);
        disable_turn(module_accessor);
        disable_dash(module_accessor);
        disable_run(module_accessor);
        disable_crouch(module_accessor);
        disable_walk(module_accessor);
        */
        }
    }
}

pub unsafe fn disable_dash(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
}

pub unsafe fn disable_run(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
}

pub unsafe fn enable_run(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
    WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
}

pub unsafe fn disable_shield(module_accessor: &mut BattleObjectModuleAccessor, disable: bool){
    if disable{
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    }
    else{
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
        //back_stick_guard(module_accessor);
    }
}

pub unsafe fn enable_ad(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
}

pub unsafe fn enable_jump(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
}

pub unsafe fn disable_jump(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
}

pub unsafe fn disable_walk(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
}

static mut enable: bool = false;

pub unsafe fn disable_crouch(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
}

pub unsafe fn enable_crouch(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
}

pub unsafe fn disable_catch(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
}
pub unsafe fn disable_turn(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    //WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_RUN_BRAKE);
}

pub unsafe fn disable_specials(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
}

pub unsafe fn enable_specials(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
}

pub unsafe fn enable_special_s(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
}

pub unsafe fn enable_special_hi(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
}

pub unsafe fn enable_special_lw(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
}

pub unsafe fn enable_special_n(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
}

pub unsafe fn disable_special_s(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
}

pub unsafe fn disable_special_hi(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
}

pub unsafe fn disable_special_lw(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
}

pub unsafe fn disable_special_n(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
}

pub unsafe fn disable_tilts(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
}

pub unsafe fn disable_ground_dodge(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
}

pub unsafe fn enable_ground_dodge(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
}

pub unsafe fn enable_tilts(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
}

pub unsafe fn disable_jab(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
}

pub unsafe fn disable_jab_100(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100);
}

pub unsafe fn enable_all(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    //WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    //WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    //WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
    //WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    enable_dash_force(module_accessor);
}
pub unsafe fn enable_jab(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
}
pub unsafe fn disable_smash_atks(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
}


pub unsafe fn disable_aerials(module_accessor: &mut BattleObjectModuleAccessor, disable:bool){
    if disable{
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    }
    else{
        enable_aerials(module_accessor);
    }
}

pub unsafe fn enable_aerials(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
}
static mut BCK_FRAME_COUNTER:[f32;8] = [0.0;8];

pub unsafe fn is_back_flick(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    let mut ENTRY_ID = get_entry_id(module_accessor);    if IS_LEFT[ENTRY_ID]{
        if ControlModule::get_stick_x(module_accessor) == 1.0{
            BCK_FRAME_COUNTER[ENTRY_ID] += 1.0;
        }
        if BCK_FRAME_COUNTER[ENTRY_ID] < 6.0{
            BCK_FRAME_COUNTER[ENTRY_ID] = 0.0;
            return true;
        }
        return false;
    }
    else{
        if ControlModule::get_stick_x(module_accessor) == -1.0{
            BCK_FRAME_COUNTER[ENTRY_ID] += 1.0;
        }
        if BCK_FRAME_COUNTER[ENTRY_ID] < 6.0{
            BCK_FRAME_COUNTER[ENTRY_ID] = 0.0;
            return true;
        }
        return false;
    }
}
pub unsafe fn enable_tilts_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 && !is_damage_check(module_accessor, false) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 && !is_damage_check(module_accessor, false) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 && !is_damage_check(module_accessor, false) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
    }
}
static mut IS_ENABLE_SPECIAL:[bool;8] = [false;8];
pub unsafe fn enable_specials_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && ![*FIGHTER_KIND_PTRAINER, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PIKMIN].contains(&smash::app::utility::get_kind(module_accessor)) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
}

pub unsafe fn enable_special_n_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
    }
}

pub unsafe fn enable_special_s_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
    }
}

pub unsafe fn enable_special_lw_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && !is_damage_check(module_accessor, false) && ![*FIGHTER_KIND_PTRAINER, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PIKMIN].contains(&smash::app::utility::get_kind(module_accessor)) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
}

pub unsafe fn enable_special_hi_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
    }
}

pub unsafe fn enable_attack_n_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK, true);
    }
}

pub unsafe fn enable_catch_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
    }
}

pub unsafe fn enable_escape_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE, true);
    }
}

pub unsafe fn enable_aerials_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if is_atk_air_input(module_accessor) {
        change_status(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR);
    }
}


pub unsafe fn enable_escape_b_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_B, true);
    }
}

pub unsafe fn enable_escape_f_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_F, true);
    }
}

pub unsafe fn enable_escape_air_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
    }
}

pub unsafe fn enable_grab_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
    }
}

pub unsafe fn enable_smash_atk_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);
    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 && !is_damage_check(module_accessor, false) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 && !is_damage_check(module_accessor, false) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 && !is_damage_check(module_accessor, false) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
    }
}

pub unsafe fn enable_guard_force(module_accessor: &mut BattleObjectModuleAccessor){
    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
        change_status(module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON);
    }
}
pub fn is_grounded(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind;
    unsafe {
        situation_kind = StatusModule::situation_kind(module_accessor) as i32;
    }
    situation_kind == *SITUATION_KIND_GROUND
}

pub unsafe fn is_smash_attack_status(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4 ||
        StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI4 ||
        StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW4
}

pub unsafe fn tech_everything(module_accessor: &mut BattleObjectModuleAccessor){
        let mut ATTACKER_ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
        let mut ENTRY_ID = get_entry_id(module_accessor);
        if is_smash_attack_status(module_accessor) && is_inflic(module_accessor){
            IS_ATK_SMASH_HIT = true;
            SMASH_HIT_ENTRY_ID = ENTRY_ID as i32;
        }
        if !IS_ATK_SMASH_HIT{
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR{
                TECH_FRAME[ENTRY_ID]+=1;
            } else {
                TECH_FRAME[ENTRY_ID] = 0;
            }
            if TECH_FRAME[ENTRY_ID]<=15 && TECH_FRAME[ENTRY_ID]>0 && !WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DEATH_PREDICTION){
                if is_cpu(module_accessor){
                    change_status(module_accessor, *FIGHTER_STATUS_KIND_PASSIVE_WALL);
                }
                else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
                    change_status(module_accessor, *FIGHTER_STATUS_KIND_PASSIVE_WALL);
                }
            }
            else if TECH_FRAME[ENTRY_ID]<=20 && TECH_FRAME[ENTRY_ID]>0 && WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DEATH_PREDICTION){
                if is_cpu(module_accessor){
                    change_status(module_accessor, *FIGHTER_STATUS_KIND_PASSIVE_WALL);
                }
                else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
                    change_status(module_accessor, *FIGHTER_STATUS_KIND_PASSIVE_WALL);
                }
            }
        }
    else if ENTRY_ID != SMASH_HIT_ENTRY_ID as usize && !is_damage_check(module_accessor, false){
        IS_ATK_SMASH_HIT = false;
    }
}


pub unsafe fn no_lag_shield(module_accessor: &mut BattleObjectModuleAccessor){
    let status_kind = StatusModule::status_kind(module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF{
        CancelModule::enable_cancel(module_accessor);
    }
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) && !is_damage_check(module_accessor, false){
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    }

}

pub unsafe fn get_cancel_frame(module_accessor: &mut BattleObjectModuleAccessor) -> f32{
    FighterMotionModuleImpl::get_cancel_frame(module_accessor, smash::phx::Hash40::new_raw(MotionModule::motion_kind(module_accessor)), true) as f32
}

pub unsafe fn get_current_frame(module_accessor: &mut BattleObjectModuleAccessor) -> f32 {
    MotionModule::frame(module_accessor)
}

static mut IS_ALLOW_AD_ATK_AIR:[bool;8] = [false;8];
pub unsafe fn ad_cancels(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(module_accessor);
    enable_jump(module_accessor);
    if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_GROUND || StatusModule::situation_kind(module_accessor) == SITUATION_KIND_CLIFF{
        AIRDODGE[ENTRY_ID] = 2;
    }
    if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR{
        AIRDODGE[ENTRY_ID] += 2;
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR{
            IS_ALLOW_AD_ATK_AIR[ENTRY_ID] = true;
        }
    }
    else{
        IS_ALLOW_AD_ATK_AIR[ENTRY_ID] =false;
    }
    if status_kind == *FIGHTER_STATUS_KIND_TREAD_JUMP{
        AIRDODGE[ENTRY_ID]+=1;
    }
    if AIRDODGE[ENTRY_ID] >= 4 {
        AIRDODGE[ENTRY_ID] = 4;
    }
    if AIRDODGE[ENTRY_ID] > 0{
        if !IS_ALLOW_AD_ATK_AIR[ENTRY_ID]{
            WorkModule::set_float(module_accessor, 1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_AIR);
            WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        }
        else{
            WorkModule::set_float(module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_AIR);
            WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        }
        if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR{
            WorkModule::set_float(module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_AIR);
            WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        }

    }
    if AIRDODGE[ENTRY_ID] <= 0{
        AIRDODGE[ENTRY_ID] = 0;
    }
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind){
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        CancelModule::enable_cancel(module_accessor);
        if get_current_frame(module_accessor) == 1.0{
            AIRDODGE[ENTRY_ID] -= 1;
        }
        if AIRDODGE[ENTRY_ID]>0 && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) && get_current_frame(module_accessor) > end_frame(module_accessor) / 3.0
        {
            change_status(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR);
        }
    }
    if CancelModule::is_enable_cancel(module_accessor) && StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR && ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_GUARD)
        && AIRDODGE[ENTRY_ID]>0 && status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR{
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
    }
    if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F].contains(&status_kind){
        let cancelframe_d = FighterMotionModuleImpl::get_cancel_frame(module_accessor, smash::phx::Hash40::new_raw(MotionModule::motion_kind(module_accessor)), true) as f32;
        if MotionModule::frame(module_accessor) >= cancelframe_d {
            CancelModule::enable_cancel(module_accessor);
        }
    }
}
static mut FRAME_TIMER:[f32;8] = [0.0; 8];
static mut FRAME_START:[bool;8] = [false; 8];
static mut PREV_STATUS_1:[i32;8] = [0;8];
static mut PREV_STATUS_2:[i32;8] = [0;8];
static mut PREV_STATUS_3:[i32;8] = [0;8];
static mut PREV_STATUS_4:[i32;8] = [0;8];
static mut PREV_STATUS_5:[i32;8] = [0;8];
static mut PREV_STATUS_6:[i32;8] = [0;8];
static mut PREV_STATUS_7:[i32;8] = [0;8];
static mut PREV_STATUS_8:[i32;8] = [0;8];
static mut PREV_STATUS_9:[i32;8] = [0;8];
static mut PREV_STATUS_10:[i32;8] = [0;8];
static mut prev_count: i32 = 0;
static mut CAN_CANCEL:bool = false;

pub unsafe fn enable_jump_force(module_accessor: &mut BattleObjectModuleAccessor, forced: bool){
    if forced{
        if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR{
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
        }
        else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND{
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }
        if ControlModule::is_enable_flick_jump(module_accessor){
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            }
            else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
        }
    }
    else{
        if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
            else if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            }
        }
        if ControlModule::is_enable_flick_jump(module_accessor){
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
                else if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                }
            }
        }
    }

}

pub unsafe fn dash_attack(fighter: &mut L2CFighterCommon){
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let status_kind = StatusModule::status_kind(module_accessor);
    let mul_momentum  = smash::phx::Vector3f { x: 2.0, y: 1.0, z: 1.0 };
    let dmg_succeed = WorkModule::get_float(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) * 10000.0;
    let x_vel = KineticModule::get_sum_speed_x(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let ENTRY_ID = get_entry_id(module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH{
        if MotionModule::frame(module_accessor) == 1.0{
            DASH_START_X[ENTRY_ID] = PostureModule::pos_x(module_accessor);
        }
        CancelModule::enable_cancel(module_accessor);
        if IS_USED_DATK_SUPER_ARMOR[ENTRY_ID]{
            acmd!(lua_state, {
                sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
            });
        }
        else{
            acmd!(lua_state, {
                sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
            });
            if dmg_succeed > 0.0{
                IS_USED_DATK_SUPER_ARMOR[ENTRY_ID] = true;
            }
        }
        disable_walk(module_accessor);
        disable_dash(module_accessor);
        disable_run(module_accessor);
        disable_turn(module_accessor);
        disable_crouch(module_accessor);
        if MotionModule::frame(module_accessor) == 1.0{
            KineticModule::add_speed(module_accessor, &phx::Vector3f{
                x: 0.8,
                y: 0.0,
                z: 0.0
            });
        }
        if ControlModule::get_stick_y(module_accessor) < -0.5{
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);
        }
    }
    else{
        DASH_START_X[ENTRY_ID] = 0.0;
        IS_USED_DATK_SUPER_ARMOR[ENTRY_ID] = false;
    }
}

pub fn crouch_jump(module_accessor: &mut BattleObjectModuleAccessor){
    unsafe{
        let status_kind = StatusModule::status_kind(module_accessor);
        let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let stick_x = ControlModule::get_stick_x(module_accessor);
        let mut jump_x = 0.0;
        if stick_x > 0.5{
            jump_x = 1.0;
        }
        else if stick_x < -0.5{
            jump_x = -1.0;
        }
        else{
            jump_x = 0.0;
        }
        if [*FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_B,
            *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_RV].contains(&StatusModule::prev_status_kind(module_accessor, 1)) &&
            status_kind == *FIGHTER_STATUS_KIND_JUMP{
            if MotionModule::frame(module_accessor) <= 5.0{
                PostureModule::set_pos(module_accessor, &smash::phx::Vector3f {
                    x: PostureModule::pos_x(module_accessor) + jump_x,
                    y: PostureModule::pos_y(module_accessor) + 6.0,
                    z: PostureModule::pos_z(module_accessor)
                });
            }
        }
    }
}


pub unsafe fn aerial_smash_atks(module_accessor: &mut BattleObjectModuleAccessor){
    let status_kind = StatusModule::status_kind(module_accessor);
    let mut ENTRY_ID = get_entry_id(module_accessor);
    if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SMASH) &&
          !is_grounded(module_accessor) &&
        !is_damage_check(module_accessor, false){
            if ControlModule::get_stick_y(module_accessor) > 0.5{
                change_status(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD);
            }
            else if ControlModule::get_stick_y(module_accessor) < -0.5 {
                change_status(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD);
            }
            else{
                change_status(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD);
            }
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind){
        StatusModule::set_situation_kind(module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), true);
    }
}


static mut TOTAL_DASHES_INPUTS:[i32;8] = [0;8];
static mut ALLOW_DASH:[bool;8] = [false;8];
static mut IS_DASH_INPUT:[bool;8] = [false;8];

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn fighter_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ENTRY_ID = get_entry_id(&mut *fighter.module_accessor);
    if !ALLOW_DASH[ENTRY_ID]{
        fighter.sub_shift_status_main(L2CValue::Ptr(run as *const () as _))
    }
    else{
        ALLOW_DASH[ENTRY_ID] = false;
        call_original!(fighter)
    }
}

unsafe extern "C" fn run(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), false.into());
    L2CValue::I32(0)
}
static mut IS_TURN_DASH:[bool;8] = [false;8];
static mut WAIT_FRAME_COUNTER:[f32;8] = [0.0;8];
static mut FLICK_FRAME_COUNTER:[f32;8] = [0.0;8];
static mut STICK_X:[f32;8] = [0.0;8];
static mut IS_DASH_BACK_RIGHT:[bool;8] = [false;8];
static mut IS_DASH_BACK_LEFT:[bool;8] = [false;8];
static mut IS_DASH_BACK:[bool;8] = [false;8];
static mut CAN_DASH:[bool;8] = [false;8];

pub unsafe fn get_entry_count() -> i32 {
    FighterManager::entry_count(get_fighter_manager())
}

pub unsafe fn enable_dash_force(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    let vel_3f = KineticModule::get_sum_speed3f(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    let dash_speed: f32 = WorkModule::get_param_float(module_accessor, hash40("dash_speed"), 0);
    let mut pivot_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: dash_speed * -0.75, y: 0.0, z: 0.0};
    let fighter_kind = smash::app::utility::get_kind(module_accessor);

    if ![*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_JUMP,
        *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind){
        if !ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && ![*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_JUMP,
            *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind)
        && ![*FIGHTER_DONKEY_STATUS_KIND_SHOULDER_END,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT_B,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_PASS,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK].contains(&status_kind){
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
                if !is_damage_check(module_accessor, false) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER, smash::app::FighterEntryID(ENTRY_ID as i32))) {
                    enable_jump_force(module_accessor, false);
                    if [*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_DEMON].contains(&fighter_kind){
                        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0{
                            change_status(module_accessor, *FIGHTER_STATUS_KIND_DASH);
                        }
                        else{
                            if [*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN].contains(&fighter_kind){
                                change_status(module_accessor, *FIGHTER_RYU_STATUS_KIND_DASH_BACK);
                            }
                            else if [*FIGHTER_KIND_DOLLY].contains(&fighter_kind){
                                change_status(module_accessor, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK);
                            }
                            else if [*FIGHTER_KIND_DEMON].contains(&fighter_kind){
                                change_status(module_accessor, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK);
                            }
                        }
                    }
                    else{
                        if !is_training_mode() && get_entry_count() > 2 && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0{
                            change_status(module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH);
                        }
                        else{
                            change_status(module_accessor, *FIGHTER_STATUS_KIND_DASH);
                        }
                    }
                    if get_entry_count() <= 2 && ControlModule::get_stick_x(module_accessor) == BACKWARDS_LR[ENTRY_ID] &&
                        ![*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_DEMON].contains(&fighter_kind) {
                        IS_DASH_BACK[ENTRY_ID] = true;
                    }
                    disable_smash_atks(module_accessor);
                    disable_tilts(module_accessor);
                    disable_dash(module_accessor);
                    disable_walk(module_accessor);
                    disable_run(module_accessor);
                    disable_jab(module_accessor);
                    disable_turn(module_accessor);
                }
            }
        }
    }

    if is_training_mode() && [*FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) &&
        ![*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_DEMON].contains(&fighter_kind){
        IS_DASH_BACK[ENTRY_ID] = true;
        change_status(module_accessor, *FIGHTER_STATUS_KIND_DASH);
    }
    else if get_entry_count() <= 2 && [*FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) &&
        ![*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_DEMON].contains(&fighter_kind){
        IS_DASH_BACK[ENTRY_ID] = true;
        change_status(module_accessor, *FIGHTER_STATUS_KIND_DASH);
    }
    if [*FIGHTER_STATUS_KIND_DASH].contains(&status_kind){
        if IS_DASH_BACK[ENTRY_ID]{
            KineticModule::clear_speed_all(module_accessor);
            KineticModule::add_speed(module_accessor, &pivot_boost);
        }
        WAIT_FRAME_COUNTER[ENTRY_ID] +=1.0;
        if ControlModule::get_stick_y(module_accessor) >= 0.7{
            if WAIT_FRAME_COUNTER[ENTRY_ID] < 5.0{
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            }
            else{
                WAIT_FRAME_COUNTER[ENTRY_ID] = 0.0;
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            }
            change_status(module_accessor, *FIGHTER_STATUS_KIND_WAIT);
        }
    }
    else if [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK].contains(&status_kind){
        if ControlModule::get_stick_y(module_accessor) >= 0.7{
            if WAIT_FRAME_COUNTER[ENTRY_ID] < 5.0{
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            }
            else{
                WAIT_FRAME_COUNTER[ENTRY_ID] = 0.0;
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            }
            change_status(module_accessor, *FIGHTER_STATUS_KIND_WAIT);
        }
    }
}


pub unsafe fn walk_stuff(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = get_entry_id(module_accessor);    let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
    let vel_3f = KineticModule::get_sum_speed3f(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    let FIGHTER_MANAGER = get_fighter_manager();
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    let walk_speed_max: f32 = WorkModule::get_param_float(module_accessor, hash40("walk_accel_max"), 0);
    let pivot_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: (walk_speed_max * -0.75), y: 0.0, z: 0.0};
    if status_kind == *FIGHTER_STATUS_KIND_TURN && ControlModule::get_stick_x(module_accessor) > 0.0 && PostureModule::lr(module_accessor) == -1.0{
        change_status(module_accessor, *FIGHTER_STATUS_KIND_WALK);
        if PostureModule::lr(module_accessor) != -1.0{
            PostureModule::set_lr(module_accessor, -1.0);
            PostureModule::update_rot_y_lr(module_accessor);
        }
        KineticModule::clear_speed_all(module_accessor);
        KineticModule::add_speed(module_accessor, &pivot_boost);
    }
    if status_kind == *FIGHTER_STATUS_KIND_TURN && ControlModule::get_stick_x(module_accessor) < 0.0 && PostureModule::lr(module_accessor) == 1.0{
        change_status(module_accessor, *FIGHTER_STATUS_KIND_WALK);
        if PostureModule::lr(module_accessor) != 1.0{
            PostureModule::set_lr(module_accessor, 1.0);
            PostureModule::update_rot_y_lr(module_accessor);
        }
        KineticModule::clear_speed_all(module_accessor);
        KineticModule::add_speed(module_accessor, &pivot_boost);
    }

}


pub unsafe fn is_inflic(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)
}

pub unsafe fn is_inflic_any(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL)
}
pub unsafe fn off_the_top_sd(module_accessor: &mut BattleObjectModuleAccessor){
    if PostureModule::pos_y(module_accessor) > 180.0{
        if StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_DEAD{
            //IS_DEAD[ENTRY_ID] = true;
            PostureModule::add_pos_2d(module_accessor, &smash::phx::Vector2f{x: 0.0, y: -5.0});
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
        }
    }
}




static mut DIRECTION_FACING:[f32;8] = [0.0;8];
static mut FRAME_COUNTER_SPECIAL_HI:[f32;8] = [0.0;8];
static mut IS_GUARD_ON:[bool;8] = [false;8];
static mut IS_ALLOWED_SPECIAL_HI:[bool;8] = [false;8];
static mut SPECIAL_FRAME_COUNTER : [f32;8] = [0.0;8];


pub unsafe fn critical_zoom(fighter: &mut L2CFighterCommon, rate : u8, frames : f32, zoom : f32) {
    let lua_state = fighter.lua_state_agent;
    if SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_common_finishhit")) {
        acmd!(lua_state, {
            EFFECT(hash40("sys_bg_criticalhit"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
             CAM_ZOOM_IN_arg5(frames, 0.0, zoom, 0.0, 0.0);
        });
        if rate != 0 {
            SlowModule::set_whole(fighter.module_accessor, rate, 0);
        }
    }
    else{
        acmd!(lua_state, {
            PLAY_SE(hash40("se_common_finishhit"));
        });
    }
}

#[smashline::fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe{
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut ENTRY_ID = get_entry_id(module_accessor);
        IS_DASH_BACK[ENTRY_ID] = false;
        AIR_TIME_COUNTER[ENTRY_ID] = 0.0;
        JUMP_HOLD_TIMER[ENTRY_ID] = 0.0;
        IS_FLIGHT[ENTRY_ID] = false;
    }
}

pub unsafe fn is_cpu(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    let mut ENTRY_ID = get_entry_id(module_accessor) as i32;
    FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(get_fighter_manager(),smash::app::FighterEntryID(ENTRY_ID)))
}

#[smashline::fighter_frame_callback]
pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(module_accessor, Hash40::new_raw(MotionModule::motion_kind(module_accessor)), true) as f32;
        let mut ENTRY_ID = get_entry_id(module_accessor);
        let dmg_succeed = WorkModule::get_float(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) * 10000.0;
        DIRECTION_FACING[ENTRY_ID] = PostureModule::lr(module_accessor);

        let FIGHTER_MANAGER = get_fighter_manager();
        if [*FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind){
            MotionModule::set_rate(module_accessor, 2.0);
        }

        if  [*FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F].contains(&status_kind) &&
            (StatusModule::situation_kind(get_module_accessor(0)) == *SITUATION_KIND_GROUND &&
        StatusModule::situation_kind(get_module_accessor(1)) == *SITUATION_KIND_GROUND){
            if ENTRY_ID == 0{
                if POSX[ENTRY_ID] < POSX[1] {
                    if POSX[1] - POSX[ENTRY_ID] < 6.0{
                        acmd!(lua_state, {
                            SLOW_OPPONENT(3, 17.5)
                        });
                    }
                }
                else {
                    if  POSX[ENTRY_ID] - POSX[1] < 6.0{
                        acmd!(lua_state, {
                            SLOW_OPPONENT(3, 17.5)
                        });
                    }
                }
            }
            if ENTRY_ID == 1{
                if POSX[ENTRY_ID] < POSX[0] {
                    if POSX[0] - POSX[ENTRY_ID] < 6.0{
                        acmd!(lua_state, {
                            SLOW_OPPONENT(3, 17.5)
                        });
                    }
                }
                else {
                    if  POSX[ENTRY_ID] - POSX[0] < 6.0{
                        acmd!(lua_state, {
                            SLOW_OPPONENT(3, 17.5)
                        });
                    }
                }
            }
        }
        if  [*FIGHTER_STATUS_KIND_ESCAPE].contains(&status_kind) &&
            (StatusModule::situation_kind(get_module_accessor(0)) == *SITUATION_KIND_GROUND &&
                StatusModule::situation_kind(get_module_accessor(1)) == *SITUATION_KIND_GROUND){
            if ENTRY_ID == 0{
                if POSX[ENTRY_ID] < POSX[1] {
                    if POSX[1] - POSX[ENTRY_ID] < 7.0{
                        acmd!(lua_state, {
                            SLOW_OPPONENT(3, 17.5)
                        });
                    }
                }
                else {
                    if  POSX[ENTRY_ID] - POSX[1] < 7.0{
                        acmd!(lua_state, {
                            SLOW_OPPONENT(3, 17.5)
                        });
                    }
                }
            }
            if ENTRY_ID == 1{
                if POSX[ENTRY_ID] < POSX[0] {
                    if POSX[0] - POSX[ENTRY_ID] < 7.0{
                        acmd!(lua_state, {
                            SLOW_OPPONENT(3, 17.5)
                        });
                    }
                }
                else {
                    if  POSX[ENTRY_ID] - POSX[0] < 7.0{
                        acmd!(lua_state, {
                            SLOW_OPPONENT(3, 17.5)
                        });
                    }
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_PASS{
            MotionModule::set_rate(module_accessor, 2.0);
        }
        if !FighterManager::is_result_mode(FIGHTER_MANAGER) && StatusModule::situation_kind(module_accessor) != *SITUATION_KIND_CLIFF &&
            !is_anyplayer_final(){
            if is_inflic_any(module_accessor) || is_damage_check(module_accessor, false){
                AIR_TIME_COUNTER[ENTRY_ID] = 0.0;
                CAN_TIMER_COUNT[ENTRY_ID] = false;
            }
            else if !is_grounded(module_accessor) || [*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4,
                *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
                    *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD].contains(&status_kind){
                CAN_TIMER_COUNT[ENTRY_ID] = true;
            }
            if CAN_TIMER_COUNT[ENTRY_ID]{
                AIR_TIME_COUNTER[ENTRY_ID] += 1.0;
            }
            if is_grounded(module_accessor) && ![*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4,
                *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
                    *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD].contains(&status_kind){
                CAN_TIMER_COUNT[ENTRY_ID] = false;
                if AIR_TIME_COUNTER[ENTRY_ID] > 0.0{
                    AIR_TIME_COUNTER[ENTRY_ID] -= 1.0;
                }
            }
            if AIR_TIME_COUNTER[ENTRY_ID] >= 300.0{
                change_status(module_accessor, *FIGHTER_STATUS_KIND_DEAD);
                AIR_TIME_COUNTER[ENTRY_ID] = 0.0;
            }
        }

        if ENTRY_ID > 8 {
            ENTRY_ID = 8;
        }
        if fighter_kind == *FIGHTER_KIND_YOSHI{
            WorkModule::set_int(module_accessor, 0, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_HOP_COUNT);
        }
        if is_damage_check(module_accessor, false) && !CaptureModule::is_capture(module_accessor) &&
            ![*FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY,
                *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_MEWTWO_THROWN].contains(&status_kind) && !is_anyplayer_final(){
            HIT_FRAME_COUNTER[ENTRY_ID] += 1.0;
            if HIT_FRAME_COUNTER[ENTRY_ID] >= 120.0{
                if is_cpu(module_accessor){
                    let rand = rand(hash40("fighter"), 4);
                    match rand {
                        1 => {
                            change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N);
                            IS_CTR_HIT[ENTRY_ID] = true;
                        }
                        2 => {
                            change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S);
                            IS_CTR_HIT[ENTRY_ID] = true;
                        }
                        3 => {
                            change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI);
                            IS_CTR_HIT[ENTRY_ID] = true;
                        }
                        4 => {
                            change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW);
                            IS_CTR_HIT[ENTRY_ID] = true;
                        }
                        _ => {}
                    }
                }
                else{
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{
                        KineticModule::clear_speed_all(module_accessor);
                        change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N);
                        IS_CTR_HIT[ENTRY_ID] = true;
                    }
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0{
                        KineticModule::clear_speed_all(module_accessor);
                        change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S);
                        IS_CTR_HIT[ENTRY_ID] = true;
                    }
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0{
                        KineticModule::clear_speed_all(module_accessor);
                        change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI);
                        IS_CTR_HIT[ENTRY_ID] = true;
                    }
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0{
                        KineticModule::clear_speed_all(module_accessor);
                        change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW);
                        IS_CTR_HIT[ENTRY_ID] = true;
                    }
                }
            }
        }
        else{
            HIT_FRAME_COUNTER[ENTRY_ID] = 0.0;
        }

        if is_special(module_accessor, false) && IS_CTR_HIT[ENTRY_ID] && CTR_HIT_INVIN[ENTRY_ID] < 60.0{
            CTR_HIT_INVIN[ENTRY_ID] += 1.0;
            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
        else if is_special(module_accessor, true) && IS_CTR_HIT[ENTRY_ID]{
            IS_CTR_HIT[ENTRY_ID] = false;
            CTR_HIT_INVIN[ENTRY_ID] = 0.0;
        }

        if is_ganon_captain_reverse_punch(module_accessor){
            PostureModule::set_lr(module_accessor, BACKWARDS_LR[ENTRY_ID]);
        }
        if is_special_hi(module_accessor, false){
            if !is_inflic(module_accessor){
                enable_aerials_force(module_accessor);
            }
            enable_escape_air_force(module_accessor);
            FRAME_COUNTER_SPECIAL_HI[ENTRY_ID] += 1.0;
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 && !is_damage_check(module_accessor, false) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32)))
                && FRAME_COUNTER_SPECIAL_HI[ENTRY_ID] > 30.0 {
                change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI);
                FRAME_COUNTER_SPECIAL_HI[ENTRY_ID] = 0.0;
            }
            enable_jump_force(module_accessor, false);
        }
        if !is_damage_check(module_accessor, false) && !is_special(module_accessor, false) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR{
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0{
                change_status(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_FALL{
            enable_specials_force(module_accessor);
        }
        if fighter_kind == *FIGHTER_KIND_DEMON && [*FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_FALL].contains(&status_kind){
            change_status(module_accessor, *FIGHTER_STATUS_KIND_FALL);
        }
        if fighter_kind == *FIGHTER_KIND_SNAKE && [*FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT].contains(&status_kind){
            change_status(module_accessor, *FIGHTER_STATUS_KIND_FALL);
        }

        if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_JUMP,
            *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind){
            disable_catch(module_accessor);
            disable_jab(module_accessor);
        }
        if !is_anyplayer_final(){
            tech_everything(module_accessor);
        }
        if smashball::is_training_mode(){
            auto_turnaround(module_accessor);
        }
        else if FighterManager::entry_count(FIGHTER_MANAGER) <= 2{
            auto_turnaround(module_accessor);
        }
        // walk_stuff(module_accessor);
        throw_cancels(module_accessor);
        ad_cancels(module_accessor);
        //off_the_top_sd(module_accessor);
        no_lag_shield(module_accessor);
        disable_turn(module_accessor);
        dash_attack(fighter);
        crouch_jump(module_accessor);
        if !(is_special(module_accessor, false) && is_inflic(module_accessor)) && !is_anyplayer_final(){
            aerial_smash_atks(module_accessor);
        }

        // StatusModule::change_status_request_from_script(module_accessor, STATUS_KIND[ENTRY_ID], true);
        if !is_damage_check(module_accessor, false) && is_grounded(module_accessor) && ![*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_END,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT_B,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_PASS,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK].contains(&status_kind){
            enable_jump(module_accessor);
            enable_dash_force(module_accessor);
        }

        if status_kind == *FIGHTER_STATUS_KIND_SQUAT && MotionModule::frame(module_accessor) < 15.0{
            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
        if [*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER].contains(&status_kind){
            //PostureModule::set_pos_2d(module_accessor, &smash::phx::Vector2f{x: 0.0, y: 0.0});
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI{
                if MotionModule::frame(module_accessor) >= MotionModule::end_frame(module_accessor) - 5.0{
                    //StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }

        }
        if [hash40("appeal_s_l"), hash40("appeal_s_r"), hash40("appeal_hi_l"), hash40("appeal_hi_r"), hash40("appeal_lw_l"), hash40("appeal_lw_r")].contains(&MotionModule::motion_kind(module_accessor)){
            if MotionModule::frame(module_accessor) <= 1.0{
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            }
            else{
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            }
        }
        if !is_anyplayer_final() && is_inflic(module_accessor){
            if ![*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_JUMP,
                *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind){
                if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100].contains(&status_kind)
                && MotionModule::frame(module_accessor) > 5.0{
                    enable_tilts_force(module_accessor);
                    CancelModule::enable_cancel(module_accessor);
                }
                else if [*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind)
                    && MotionModule::frame(module_accessor) > 10.0{
                    CancelModule::enable_cancel(module_accessor);
                }
                else if [hash40("attack_air_n"), hash40("attack_air_n_2"), hash40("attack_air_n_3")].contains(&MotionModule::motion_kind(module_accessor))
                    && MotionModule::frame(module_accessor) > 5.0{
                    CancelModule::enable_cancel(module_accessor);
                }
                else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && MotionModule::frame(module_accessor) > 10.0{
                    disable_aerials(module_accessor, true);
                    CancelModule::enable_cancel(module_accessor);
                }
                else{
                    CancelModule::enable_cancel(module_accessor);
                }
                enable = true;
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH{
                    //enable_tilts_force(module_accessor);
                    //enable_jab_force(module_accessor);
                    enable_tilts(module_accessor);
                    enable_jab(module_accessor);
                    enable_smash_atk_force(module_accessor);
                    // change_status(module_accessor, *FIGHTER_STATUS_KIND_DEAD);
                }
                else{
                    disable_jab(module_accessor);
                    disable_tilts(module_accessor);
                }
                if [*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4,
                    *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START].contains(&status_kind){
                    disable_smash_atks(module_accessor);
                    disable_jab(module_accessor);
                    disable_tilts(module_accessor);
                }
                enable_jump_force(module_accessor, true);
                if is_special(module_accessor, false){
                    disable_jab(module_accessor);
                    disable_jab_100(module_accessor);
                    disable_smash_atks(module_accessor);
                    disable_tilts(module_accessor);
                    disable_specials(module_accessor);
                    disable_aerials(module_accessor, true);
                    if !is_sword_dance(module_accessor) &&
                        !([*FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_ROY, *FIGHTER_KIND_CHROM].contains(&fighter_kind) && [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind)){
                        enable_special_hi_force(module_accessor);
                        enable_special_s_force(module_accessor);
                        enable_special_lw_force(module_accessor);
                    }
                }
                else{
                    enable_specials_force(module_accessor);
                }
                /*
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR{
                    HIT_FRAME[ENTRY_ID] += 1.0;
                    if HIT_FRAME[ENTRY_ID] < 20.0{
                        KineticModule::unable_energy(module_accessor,  *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    }
                    else{
                        KineticModule::enable_energy(module_accessor,  *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    }
                }
                 */
                enable_dash_force(module_accessor);
                if !is_special(module_accessor, false) && status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR{
                    enable_aerials(module_accessor);
                }
                else{
                    disable_aerials(module_accessor, true);
                }

            }
        }
        else {
            HIT_FRAME[ENTRY_ID] = 0.0;
            if [*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) {
                disable_shield(module_accessor, true);
            }
        }
        if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT){
        }
        if status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK && IS_USED_PUMMEL[ENTRY_ID]{
           // change_status(module_accessor, *FIGHTER_STATUS_KIND_CATCH_WAIT);
        }
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD){
            if [*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3,
                *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind){
                enable_dash_force(module_accessor);
                enable_jump_force(module_accessor, false);
                enable_ad(module_accessor);
            }
            if [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind){
                enable_ground_dodge(module_accessor);
                enable_ad(module_accessor);
            }
        }



        if [*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) {
            enable_grab_force(module_accessor);
            disable_walk(module_accessor);
                if ControlModule::get_stick_x(module_accessor) == BACKWARDS_LR[ENTRY_ID] ||
                    ControlModule::get_sub_stick_x(module_accessor) == BACKWARDS_LR[ENTRY_ID]{
                    IS_BACK_TILT[ENTRY_ID] = true;
                }
            if is_inflic(module_accessor) {
                enable_smash_atk_force(module_accessor);
                disable_shield(module_accessor, false);
                if MotionModule::frame(module_accessor) >= 10.0 {
                    CancelModule::enable_cancel(module_accessor);
                    enable_all(module_accessor);
                    disable_jab(module_accessor);
                }
            }
        }
        else{
            IS_BACK_TILT[ENTRY_ID] = false;
        }

        if [*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_100].contains(&status_kind) {
            //enable_jump(module_accessor);
            CancelModule::enable_cancel(module_accessor);
            disable_jab(module_accessor);
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_100{
                disable_jab_100(module_accessor);
            }
            disable_shield(module_accessor, false)
        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
            //FighterManager::set_position_lock(FIGHTER_MANAGER, smash::app::FighterEntryID(ENTRY_ID as i32), true);
        }
        else{
            //FighterManager::set_position_lock(FIGHTER_MANAGER, smash::app::FighterEntryID(ENTRY_ID as i32), false);
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START].contains(&status_kind){
            if ControlModule::get_stick_x(module_accessor) == BACKWARDS_LR[ENTRY_ID]{
                IS_BACK_SMASH[ENTRY_ID] = true;
            }
        }
        else{
            IS_BACK_SMASH[ENTRY_ID] = false;
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind){
            CancelModule::enable_cancel(module_accessor);
            AttackModule::set_power_mul(module_accessor, 0.8);
            if IS_USED_SMASH_SUPER_ARMOR[ENTRY_ID]{
                acmd!(lua_state, {
                    sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
                });
            }
            else{
                acmd!(lua_state, {
                    sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
                });
                if dmg_succeed > 0.0{
                    IS_USED_SMASH_SUPER_ARMOR[ENTRY_ID] = true;
                }
            }
            if !is_inflic(module_accessor){
                disable_crouch(module_accessor);
            }
            else{
                enable_crouch(module_accessor);
            }
            if MotionModule::frame(module_accessor) < cancel_frame && !is_inflic(module_accessor){
                disable_jump(module_accessor);
                disable_smash_atks(module_accessor);
                disable_tilts(module_accessor);
                disable_shield(module_accessor, true);
                disable_aerials(module_accessor, true);
                disable_jab(module_accessor);
                disable_specials(module_accessor);
                disable_walk(module_accessor);
                disable_dash(module_accessor);
                disable_run(module_accessor);
                disable_turn(module_accessor);
                disable_jab_100(module_accessor);
                disable_ground_dodge(module_accessor);
            }
            else if !is_inflic(module_accessor){
                enable_all(module_accessor);
            }
        }
        else{
            IS_USED_SMASH_SUPER_ARMOR[ENTRY_ID] = false;
        }
        if [*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_3,
            *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_4, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_5, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_6].contains(&status_kind){
           enable_catch_force(module_accessor);
        }
        if is_special(module_accessor, false) &&
        ![*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_3,
                *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_4, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_5, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_6,
            *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F].contains(&status_kind){
            if !is_sword_dance(module_accessor){
                if !is_special_lw(module_accessor, false){
                    enable_special_lw_force(module_accessor);
                }
                if !is_special_s(module_accessor, false){
                    enable_special_s_force(module_accessor);
                }
                if !is_special_hi(module_accessor, false){
                    enable_special_hi_force(module_accessor);
                }
                if is_special_n(module_accessor, false){
                    enable_special_hi_force(module_accessor);
                    enable_special_s_force(module_accessor);
                    enable_special_lw_force(module_accessor);
                }
            }
            if !(fighter_kind == *FIGHTER_KIND_KIRBY && is_special_s(module_accessor, false)){
                enable_jump_force(module_accessor, false);
            }
            if is_grounded(module_accessor){
                enable_catch_force( module_accessor);
                enable_smash_atk_force(module_accessor);
                enable_tilts_force(module_accessor);
                enable_attack_n_force(module_accessor);
                if !is_inflic(module_accessor){
                    enable_tilts_force(module_accessor);
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) == 0 && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) == 0 && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
                        (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) == 0 && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0 && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0{
                        enable_attack_n_force(module_accessor);
                    }
                }
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B) == 0 && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F) == 0 && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) == 0{
                    enable_guard_force(module_accessor);
                }
                enable_escape_f_force(module_accessor);
                enable_escape_b_force(module_accessor);
                enable_escape_force(module_accessor);
                enable_dash_force(module_accessor);
            }
            else{
                if !is_inflic(module_accessor){

                }
                enable_aerials_force(module_accessor);
                enable_escape_air_force(module_accessor);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if is_inflic(module_accessor){
                if is_atk_air_input(module_accessor){
                    NUM_AERIALS[ENTRY_ID] +=1;
                }
                if NUM_AERIALS[ENTRY_ID] < 3{
                    enable_aerials_force(module_accessor);
                }
            }
            else if [hash40("attack_air_n"), hash40("attack_air_n_2"), hash40("attack_air_n_3")].contains(&MotionModule::motion_kind(module_accessor)){
                enable_jump_force(module_accessor, true);
                enable_escape_air_force(module_accessor);
            }
            enable_specials_force(module_accessor);
        }
        else{
            NUM_AERIALS[ENTRY_ID] = 0;
        }
        if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF{
            CancelModule::enable_cancel(module_accessor);
        }
        if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END{
            if MotionModule::frame(module_accessor) == MotionModule::end_frame(module_accessor){
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL{
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        }
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE,
            *FIGHTER_RYU_STATUS_KIND_DASH_BACK, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK].contains(&status_kind)  {
            if fighter_kind == *FIGHTER_KIND_WIIFIT{
                MotionModule::set_rate(module_accessor, 1.7);
            }
            else{
                MotionModule::set_rate(module_accessor, 2.0);
            }
            enable_jump_force(module_accessor, false);
            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL{
            CancelModule::enable_cancel(module_accessor);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind){
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            }
            if ControlModule::get_stick_y(module_accessor) == -1.0 && MotionModule::frame(module_accessor) > 15.0{
                //change_status(module_accessor, *FIGHTER_STATUS_KIND_SQUAT);
            }

        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            let stop_rise = smash::phx::Vector3f { x: 1.0, y: 0.0, z: 1.0 };
            KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::mul_accel(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        } else {
            let stop_rise = smash::phx::Vector3f { x: 1.0, y: 1.0, z: 1.0 };
            KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::mul_accel(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyGravity>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY));
        let y_vel = KineticModule::get_sum_speed_y(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if !is_grounded(module_accessor) && (is_special_s(module_accessor, false) || is_special_lw(module_accessor, false) || is_special_n(module_accessor, false)){
            FighterKineticEnergyGravity::set_gravity_coefficient(fighter_kinetic_energy_motion, 1.7)
        }
        else if !is_grounded(module_accessor) && is_special_hi(module_accessor, false){
            FighterKineticEnergyGravity::set_gravity_coefficient(fighter_kinetic_energy_motion, 0.9)
        }
        else {
            FighterKineticEnergyGravity::set_gravity_coefficient(fighter_kinetic_energy_motion, 1.1)
        }
        if ControlModule::get_stick_y(module_accessor) > 0.5 && y_vel <= 0.0{
            FighterKineticEnergyGravity::set_gravity_coefficient(fighter_kinetic_energy_motion, 0.5)
        }
        /*
        if is_special(module_accessor, false){
            enable_jump(module_accessor);
            CancelModule::enable_cancel(module_accessor);
            //disable_smash_atks(module_accessor);
            //disable_tilts(module_accessor);
            //disable_aerials(module_accessor, true);
            //disable_jab(module_accessor);
            disable_specials(module_accessor);
            disable_walk(module_accessor);
            disable_dash(module_accessor);
            disable_run(module_accessor);
            disable_turn(module_accessor);
        }
         */
        if status_kind == *FIGHTER_STATUS_KIND_RUN {
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        }

        if is_special(module_accessor, false) || MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi_air") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi"){
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_CATCH{
            enable_jump(module_accessor);
        }
    }
}





#[smashline::installer]
pub fn install(){
    unsafe{
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            c_str("_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"));
    }
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
    smashline::install_status_scripts!(fall_status_main, dash_status_end, escape_air_main);
    skyline::install_hooks!(attack_replace);
    smashline::install_agent_resets!(fighter_reset);
}
