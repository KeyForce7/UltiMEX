use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::BattleObjectModuleAccessor;
use std::mem;
use smash::app::sv_module_access;
use smash::app::sv_battle_object;
use smash::phx::Vector4f;
use skyline::nn::ro::LookupSymbol;
use std::time::Duration;
use smash::lib::*;


static mut FIGHTER_MANAGER_ADDR: usize = 0;
static mut AIRDODGE : [i32; 9] = [2; 9];
static mut POSX : [f32; 9] = [-1.0 ; 9];
static mut OPPONENT_ID : [usize; 9] = [9; 9];
static mut AIRTAUNT_USED :[bool; 9] =[false;9];
static mut IS_LEFT :[bool;8] =[false;8];
static mut IS_RIGHT:[bool;8] =[false;8];
static mut TECH_FRAME: [i32; 9] = [0; 9];
static mut TIMES_ATTACKED:[i32;9] = [0;9];


pub unsafe fn is_damage_check(module_accessor : *mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
        || [
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_THROWN,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_STATUS_KIND_SLEEP,
        *FIGHTER_STATUS_KIND_ESCAPE_B,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
        *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND,
        *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD,
        *FIGHTER_STATUS_KIND_SWALLOWED,
        *FIGHTER_STATUS_KIND_AIR_LASSO,
        *FIGHTER_STATUS_KIND_CATCHED_REFLET,
        *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
        *FIGHTER_STATUS_KIND_MISS_FOOT,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_BURY,
        *FIGHTER_STATUS_KIND_BURY_WAIT,
        *FIGHTER_STATUS_KIND_ICE,
        *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
        *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        *FIGHTER_STATUS_KIND_DOWN_STAND,
        *FIGHTER_STATUS_KIND_DOWN_WAIT,
        *FIGHTER_STATUS_KIND_DOWN_EAT,
        *FIGHTER_STATUS_KIND_LAY_DOWN,
        *FIGHTER_STATUS_KIND_DOWN,
        *FIGHTER_STATUS_KIND_DOWN_SPOT,
        *FIGHTER_STATUS_KIND_PASSIVE,
        *FIGHTER_STATUS_KIND_PASSIVE_WALL,
        *FIGHTER_STATUS_KIND_PASSIVE_CEIL,
        *FIGHTER_STATUS_KIND_PASSIVE_FB,
        *FIGHTER_STATUS_KIND_SLIP,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_YOSHI,
        *FIGHTER_STATUS_KIND_CAPTURE_ITEM,
        *FIGHTER_STATUS_KIND_CAPTURE_BEETLE,
        *FIGHTER_STATUS_KIND_CAPTURE_DRIVER,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_THROWN,
        *FIGHTER_STATUS_KIND_BIND,

    ].contains(&StatusModule::status_kind(module_accessor)) || FighterStopModuleImpl::is_damage_stop(module_accessor) {
        return true;
    }
    else {
        return false;
    }
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_FALL_SPECIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn fall_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    fighter.sub_shift_status_main(L2CValue::Ptr(special_fall as *const () as _))
}

unsafe extern "C" fn special_fall(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    L2CValue::I32(0)
    
}


pub unsafe fn air_taunt(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

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
static mut IS_ALLOW_ESCAPE_S4:bool = false;
static mut IS_ALLOW_ESCAPE_HI4:bool = false;
static mut IS_ALLOW_ESCAPE_SPECIAL_N:bool = false;
static mut IS_ATK_S4:bool = false;
static mut IS_ATK_HI4:bool = false;
static mut IS_SPECIAL_N:bool = false;
static mut ATTACKER_POS_X:f32 = 0.0;
static mut ATTACKER_POS_Y:f32 = 0.0;
static mut COMBO_BREAK_FRAME_COUNTER:[f32;8] = [0.0;8];
static mut COMBO_BREAK_FRAME_COUNTER2:f32 = 0.0;
pub unsafe fn combo_break(module_accessor: *mut smash::app::BattleObjectModuleAccessor){
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    LookupSymbol(
        &mut FIGHTER_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),);
    let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    let stop_motion  = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

    
    if is_damage_check(module_accessor){
        COMBO_BREAK_FRAME_COUNTER[ENTRY_ID] +=1.0;
        if COMBO_BREAK_FRAME_COUNTER[ENTRY_ID] < 10.0{
             if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0{
            IS_ALLOW_ESCAPE_S4 = true;
            }
        }
        /*
        if COMBO_BREAK_FRAME_COUNTER[ENTRY_ID] < 10.0{
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0{
           IS_ALLOW_ESCAPE_HI4 = true;
           }
        }
        if COMBO_BREAK_FRAME_COUNTER[ENTRY_ID] < 10.0{
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{
           IS_ALLOW_ESCAPE_SPECIAL_N = true;
           }
        }
        */
        else{
            COMBO_BREAK_FRAME_COUNTER[ENTRY_ID] = 0.0;
        }
       
    }
    else{
        //IS_ALLOW_ESCAPE[ENTRY_ID] = false;
        FighterManager::set_position_lock(FIGHTER_MANAGER, smash::app::FighterEntryID(ENTRY_ID as i32), false);
    }
     if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){
        ATTACKER_POS_X = PostureModule::pos_x(module_accessor);
        ATTACKER_POS_Y = PostureModule::pos_y(module_accessor);
        //IS_ALLOW_ESCAPE[ENTRY_ID] = false;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4{
            IS_ATK_S4 = true;
        }
        else if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI4{
            IS_ATK_HI4 = true;
        }
        else if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N{
            IS_SPECIAL_N = true;
        }
        else{
            IS_ATK_S4 = false;
            IS_ATK_HI4 = false;
            IS_SPECIAL_N = false;
        }
    }
    if IS_ALLOW_ESCAPE_S4 && IS_ATK_S4 {
        FighterManager::set_position_lock(FIGHTER_MANAGER, smash::app::FighterEntryID(ENTRY_ID as i32), true);
        KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_TYPE_DAMAGE_FLY);
        KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_TYPE_DAMAGE_AIR);
        KineticModule::mul_speed(module_accessor, &stop_motion, *FIGHTER_KINETIC_TYPE_DAMAGE_FLY);
        KineticModule::mul_speed(module_accessor, &stop_motion, *FIGHTER_KINETIC_TYPE_DAMAGE_AIR);
        PostureModule::set_pos_2d(module_accessor, &smash::phx::Vector2f{x: ATTACKER_POS_X, y: ATTACKER_POS_Y});
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
        IS_ALLOW_ESCAPE_S4 = false;
        IS_ATK_S4 = false;
    }
    /*
    if IS_ALLOW_ESCAPE_HI4 && IS_ATK_HI4 {
        FighterManager::set_position_lock(FIGHTER_MANAGER, smash::app::FighterEntryID(ENTRY_ID as i32), true);
        KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_TYPE_DAMAGE_FLY);
        KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_TYPE_DAMAGE_AIR);
        KineticModule::mul_speed(module_accessor, &stop_motion, *FIGHTER_KINETIC_TYPE_DAMAGE_FLY);
        KineticModule::mul_speed(module_accessor, &stop_motion, *FIGHTER_KINETIC_TYPE_DAMAGE_AIR);
        PostureModule::set_pos_2d(module_accessor, &smash::phx::Vector2f{x: ATTACKER_POS_X, y: ATTACKER_POS_Y});
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);
        IS_ALLOW_ESCAPE_HI4 = false;
        IS_ATK_HI4 = false;
    }
    if IS_ALLOW_ESCAPE_SPECIAL_N && IS_SPECIAL_N {
        FighterManager::set_position_lock(FIGHTER_MANAGER, smash::app::FighterEntryID(ENTRY_ID as i32), true);
        KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_TYPE_DAMAGE_FLY);
        KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_TYPE_DAMAGE_AIR);
        KineticModule::mul_speed(module_accessor, &stop_motion, *FIGHTER_KINETIC_TYPE_DAMAGE_FLY);
        KineticModule::mul_speed(module_accessor, &stop_motion, *FIGHTER_KINETIC_TYPE_DAMAGE_AIR);
        PostureModule::set_pos_2d(module_accessor, &smash::phx::Vector2f{x: ATTACKER_POS_X, y: ATTACKER_POS_Y});
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
        IS_ALLOW_ESCAPE_SPECIAL_N = false;
        IS_SPECIAL_N = false;
    }
    */
    else{
        //IS_ALLOW_ESCAPE[ENTRY_ID] = false;
    }
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

static mut LAST_LR:[bool;8] = [false;8];
pub unsafe fn auto_turnaround(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut opponent_pos: f32 = -1.0;
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
    LookupSymbol(
        &mut FIGHTER_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),);
    let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    if FighterManager::entry_count(FIGHTER_MANAGER)==2 && !smash::app::smashball::is_training_mode(){
        

    }
    POSX[ENTRY_ID] = PostureModule::pos_x(module_accessor);
        if OPPONENT_ID[ENTRY_ID] != 9{
            opponent_pos = POSX[OPPONENT_ID[ENTRY_ID]];
        }
        if OPPONENT_ID[ENTRY_ID] == 9/*if you haven't found who the other person is*/ {
            for i in 0..8{
                if (i as usize) != ENTRY_ID && POSX[i as usize]>= 0.0{
                    opponent_pos = POSX[i as usize];
                    OPPONENT_ID[ENTRY_ID] = (i as usize);
                }
            }
        }
//&& ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN].contains(&status_kind)

        if fighter_kind != *FIGHTER_KIND_KEN && fighter_kind != *FIGHTER_KIND_RYU && fighter_kind != *FIGHTER_KIND_DOLLY{
                if POSX[ENTRY_ID] < opponent_pos && PostureModule::lr(module_accessor)  == -1.0{
               // PostureModule::reverse_lr(module_accessor);
                PostureModule::update_rot_y_lr(module_accessor);
                PostureModule::set_lr(module_accessor, 1.0);

            }else if POSX[ENTRY_ID] > opponent_pos && PostureModule::lr(module_accessor) == 1.0{
                PostureModule::set_lr(module_accessor, -1.0);
                //PostureModule::reverse_lr(module_accessor);
                PostureModule::update_rot_y_lr(module_accessor);
            }
            
        }
       
        disable_turn(module_accessor);
}

pub unsafe fn throw_cancels(module_accessor: &mut BattleObjectModuleAccessor){
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_THROW{
        if MotionModule::frame(module_accessor) > 35.0{
            CancelModule::enable_cancel(module_accessor);
            disable_jab(module_accessor);
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
}

pub unsafe fn enable_dodge(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
}

pub unsafe fn disable_run(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
}

pub unsafe fn disable_shield(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
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

pub unsafe fn disable_tilts(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
}

pub unsafe fn disable_ground_dodge(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
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
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
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


pub unsafe fn disable_aerials(module_accessor: &mut BattleObjectModuleAccessor){
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
}

pub unsafe fn enable_aerials(module_accessor: &mut BattleObjectModuleAccessor){    
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
}
static mut BCK_FRAME_COUNTER:[f32;8] = [0.0;8];

pub unsafe fn is_back_flick(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IS_LEFT[ENTRY_ID]{
        if ControlModule::get_stick_x(module_accessor) == 1.0{
            BCK_FRAME_COUNTER[ENTRY_ID] += 1.0;
        }
        if BCK_FRAME_COUNTER[ENTRY_ID] < 4.0{
            BCK_FRAME_COUNTER[ENTRY_ID] = 0.0;
            return true;
        }
        return false;
    }
    else{
        if ControlModule::get_stick_x(module_accessor) == -1.0{
            BCK_FRAME_COUNTER[ENTRY_ID] += 1.0;
        }
        if BCK_FRAME_COUNTER[ENTRY_ID] < 4.0{
            BCK_FRAME_COUNTER[ENTRY_ID] = 0.0;
            return true;
        }
        return false;
    }
}
pub unsafe fn enable_tilts_force(module_accessor: &mut BattleObjectModuleAccessor){
    LookupSymbol(
        &mut FIGHTER_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app14FighterManagerEE\
      9instance_E\u{0}"
            .as_bytes()
            .as_ptr(), );
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
    }
}

pub unsafe fn enable_specials_force(module_accessor: &mut BattleObjectModuleAccessor){
    LookupSymbol(
        &mut FIGHTER_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app14FighterManagerEE\
      9instance_E\u{0}"
            .as_bytes()
            .as_ptr(), );
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 && !is_damage_check(module_accessor) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 && !is_damage_check(module_accessor) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 && !is_damage_check(module_accessor) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && !is_damage_check(module_accessor) && ![*FIGHTER_KIND_PTRAINER, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PIKMIN].contains(&smash::app::utility::get_kind(module_accessor)) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
}

pub unsafe fn enable_smash_atk_force(module_accessor: &mut BattleObjectModuleAccessor){
    LookupSymbol(
        &mut FIGHTER_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app14FighterManagerEE\
      9instance_E\u{0}"
            .as_bytes()
            .as_ptr(), );
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);   
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
    }
}

pub unsafe fn tech_everything(module_accessor: &mut BattleObjectModuleAccessor){
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR{
        TECH_FRAME[ENTRY_ID]+=1; 
    } else {
        TECH_FRAME[ENTRY_ID] = 0;
    }
    if TECH_FRAME[ENTRY_ID]<=10 && TECH_FRAME[ENTRY_ID]>0{
        if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASSIVE_WALL, true);
        }
    }
}

pub unsafe fn whiff_stuff(module_accessor: &mut BattleObjectModuleAccessor){
    let special: [i32; 225] = [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_L, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_EAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, *ITEM_PACMANKEY_STATUS_KIND_SPECIAL_HAVE, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_BLOW, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_PASS, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_WAIT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_FIRE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BOMB, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_FOOD, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_ITEM, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP];

    let status_kind = StatusModule::status_kind(module_accessor);
    if special.contains(&status_kind) || MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi_air") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi") || AttackModule::is_attack(module_accessor, 0, false){
        //CancelModule::enable_cancel(module_accessor);
        //disable for now might revert
        //WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        //for lucario up b
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    }
    //WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
        WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
        WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
        WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
        WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);

        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }
    if [*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_100].contains(&status_kind) {
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){
            //WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
        }
        else{
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
        }
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }

}

pub unsafe fn no_lag_shield(module_accessor: &mut BattleObjectModuleAccessor){
    let status_kind = StatusModule::status_kind(module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF{
        CancelModule::enable_cancel(module_accessor);
    }
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind){
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
        AIRDODGE[ENTRY_ID] -= 1;
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }

    if CancelModule::is_enable_cancel(module_accessor) && StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR && ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_GUARD) && AIRDODGE[ENTRY_ID]>0 && status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR{
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
    }
    if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F].contains(&status_kind){
        let cancelframe_d = FighterMotionModuleImpl::get_cancel_frame(module_accessor, smash::phx::Hash40::new_raw(MotionModule::motion_kind(module_accessor)), true) as f32;
        if MotionModule::frame(module_accessor) >= cancelframe_d {
            CancelModule::enable_cancel(module_accessor);
        }
    }
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind){
        CancelModule::enable_cancel(module_accessor);
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

pub unsafe fn enable_jump_force(module_accessor: &mut BattleObjectModuleAccessor){
    if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR{
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
    }
    else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND{
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
    }
}

pub unsafe fn dash_attack(module_accessor: &mut BattleObjectModuleAccessor){
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH{
        CancelModule::enable_cancel(module_accessor);
        disable_walk(module_accessor);
        disable_dash(module_accessor);
        disable_run(module_accessor);
        disable_turn(module_accessor);
        disable_crouch(module_accessor);
    if ControlModule::get_stick_y(module_accessor) < -0.5{
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);
    }
        
}
}
static mut TOTAL_DASHES_INPUTS:[i32;8] = [0;8];
static mut ALLOW_DASH:[bool;8] = [false;8];
static mut IS_DASH_INPUT:[bool;8] = [false;8];

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn fighter_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

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

static mut WAIT_FRAME_COUNTER:[f32;8] = [0.0;8];
static mut FLICK_FRAME_COUNTER:[f32;8] = [0.0;8];
static mut IS_DASH_BACK_RIGHT:[bool;8] = [false;8];
static mut IS_DASH_BACK_LEFT:[bool;8] = [false;8];

pub unsafe fn back_dash(module_accessor: &mut BattleObjectModuleAccessor){
    PostureModule::add_pos_2d(module_accessor, &smash::phx::Vector2f{x: -5.0, y: 0.0});
    //MotionModule::change_motion(module_accessor, smash::phx::Hash40{hash: hash40("dash")}, 0.0, 1.0, false, 0.0, false, false);
}

pub unsafe fn enable_dash_force(module_accessor: &mut BattleObjectModuleAccessor){
    let special: [i32; 225] = [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_L, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_EAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, *ITEM_PACMANKEY_STATUS_KIND_SPECIAL_HAVE, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_BLOW, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_PASS, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_WAIT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_FIRE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BOMB, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_FOOD, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_ITEM, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP];
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
    let vel_3f = KineticModule::get_sum_speed3f(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    if ![*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_JUMP,
    *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind){
        let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    let dash_speed: f32 = WorkModule::get_param_float(module_accessor, hash40("dash_speed"), 0);
    let pivot_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: (dash_speed * -0.75), y: 0.0, z: 0.0};
    /*
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0{
                if !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))){
            if ![*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind){
                enable_jump_force(module_accessor);
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
            }
            disable_smash_atks(module_accessor);
            disable_tilts(module_accessor);
            //disable_aerials(module_accessor);
            //disable_specials(module_accessor);
            disable_walk(module_accessor);
            disable_dash(module_accessor);
            disable_run(module_accessor);
            disable_jab(module_accessor);
            disable_turn(module_accessor);
        }
        }
        */
        if PostureModule::lr(module_accessor) == -1.0{
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 && ControlModule::get_stick_x(module_accessor) < -0.5{
                IS_DASH_BACK_RIGHT[ENTRY_ID] = false;
                if !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))){
            if ![*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind){
                enable_jump_force(module_accessor);
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
            }
            disable_smash_atks(module_accessor);
            disable_tilts(module_accessor);
            //disable_aerials(module_accessor);
            //disable_specials(module_accessor);
            disable_walk(module_accessor);
            disable_dash(module_accessor);
            disable_run(module_accessor);
            disable_jab(module_accessor);
            disable_turn(module_accessor);
        }
        }
            else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 && ControlModule::get_stick_x(module_accessor) > 0.5{
            IS_DASH_BACK_RIGHT[ENTRY_ID] = true;
            if !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))){
        if ![*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind){
            enable_jump_force(module_accessor);
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
        }
        disable_smash_atks(module_accessor);
        disable_tilts(module_accessor);
        //disable_aerials(module_accessor);
        //disable_specials(module_accessor);
        disable_walk(module_accessor);
        disable_dash(module_accessor);
        disable_run(module_accessor);
        disable_jab(module_accessor);
        disable_turn(module_accessor);
    }
    }
        }
        else{
            IS_DASH_BACK_RIGHT[ENTRY_ID] = false;
        }
        
    if PostureModule::lr(module_accessor) == 1.0{
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 && ControlModule::get_stick_x(module_accessor) > 0.5{
            IS_DASH_BACK_LEFT[ENTRY_ID] = false;
            if !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))){
        if ![*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind){
            enable_jump_force(module_accessor);
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
        }
        disable_smash_atks(module_accessor);
        disable_tilts(module_accessor);
        //disable_aerials(module_accessor);
        //disable_specials(module_accessor);
        disable_walk(module_accessor);
        disable_dash(module_accessor);
        disable_run(module_accessor);
        disable_jab(module_accessor);
        disable_turn(module_accessor);
    }
}
        else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 && ControlModule::get_stick_x(module_accessor) < -0.5{
            IS_DASH_BACK_LEFT[ENTRY_ID] = true;
            if !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))){
        if ![*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind){
            enable_jump_force(module_accessor);
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
        }
        disable_smash_atks(module_accessor);
        disable_tilts(module_accessor);
        //disable_aerials(module_accessor);
        //disable_specials(module_accessor);
        disable_walk(module_accessor);
        disable_dash(module_accessor);
        disable_run(module_accessor);
        disable_jab(module_accessor);
        disable_turn(module_accessor);
    }
    }
    }
    else{
        IS_DASH_BACK_LEFT[ENTRY_ID] = false;
    }
    
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DASH{
        if IS_DASH_BACK_RIGHT[ENTRY_ID] && PostureModule::lr(module_accessor) < 0.0{
                KineticModule::clear_speed_all(module_accessor);
            KineticModule::add_speed(module_accessor, &pivot_boost);
            if MotionModule::frame(module_accessor) > MotionModule::end_frame(module_accessor) - 15.0{
                //IS_DASH_BACK_RIGHT[ENTRY_ID] = false;
            }
        }
        if IS_DASH_BACK_LEFT[ENTRY_ID] && PostureModule::lr(module_accessor) > 0.0{
                KineticModule::clear_speed_all(module_accessor);
            KineticModule::add_speed(module_accessor, &pivot_boost);
            if MotionModule::frame(module_accessor) > MotionModule::end_frame(module_accessor) - 15.0{
                //IS_DASH_BACK_LEFT[ENTRY_ID] = false;
            }
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

            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        }
    }
    }
    else{
        IS_DASH_BACK_LEFT[ENTRY_ID] = false;
        IS_DASH_BACK_RIGHT[ENTRY_ID] = false;
    }
}


pub unsafe fn walk_stuff(module_accessor: &mut BattleObjectModuleAccessor){
        let special: [i32; 225] = [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_L, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_EAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, *ITEM_PACMANKEY_STATUS_KIND_SPECIAL_HAVE, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_BLOW, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_PASS, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_WAIT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_FIRE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BOMB, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_FOOD, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_ITEM, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP];
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
    let vel_3f = KineticModule::get_sum_speed3f(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    let walk_speed_max: f32 = WorkModule::get_param_float(module_accessor, hash40("walk_accel_max"), 0);
    let pivot_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: (walk_speed_max * -0.75), y: 0.0, z: 0.0};
    if status_kind == *FIGHTER_STATUS_KIND_WALK && ControlModule::get_stick_x(module_accessor) > 0.0 && PostureModule::lr(module_accessor) == -1.0{
        if PostureModule::lr(module_accessor) != -1.0{
            PostureModule::set_lr(module_accessor, -1.0);
            PostureModule::update_rot_y_lr(module_accessor);
        }
        KineticModule::clear_speed_all(module_accessor);
            KineticModule::add_speed(module_accessor, &pivot_boost);
    }
    if status_kind == *FIGHTER_STATUS_KIND_WALK && ControlModule::get_stick_x(module_accessor) < 0.0 && PostureModule::lr(module_accessor) == 1.0{
        if PostureModule::lr(module_accessor) != 1.0{
            PostureModule::set_lr(module_accessor, 1.0);
            PostureModule::update_rot_y_lr(module_accessor);
        }
        KineticModule::clear_speed_all(module_accessor);
            KineticModule::add_speed(module_accessor, &pivot_boost);
    }

}
static mut IS_HIT:[bool;8] = [false;8];
static mut IS_DEAD:[bool;8] = [false;8];
static mut IS_ATTACK:[bool;8] = [false;8];

pub unsafe fn is_inflic(module_accessor: &mut BattleObjectModuleAccessor) -> bool{
    if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){
        return true;
    }
    else{
        return false;
    }
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

const dir_left:f32 = -1.0;
const dir_right:f32 = 1.0;
static mut DIRECTION_FACING:[f32;8] = [0.0;8];
#[smashline::fighter_frame_callback]
pub fn once_per_fighter_frame(fighter: &mut smash::common::root::lua2cpp::L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        DIRECTION_FACING[ENTRY_ID] = PostureModule::lr(module_accessor);
            //println!("{}", prev_status);
        let mut start_count = false;
        if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH{
            PostureModule::reverse_lr(module_accessor);
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
        }
        if status_kind == *FIGHTER_STATUS_KIND_TURN{
            //StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
        }
        if status_kind == *FIGHTER_STATUS_KIND_TURN_RUN{
            PostureModule::reverse_lr(module_accessor);
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_RUN, true);
        }
        if status_kind == *FIGHTER_STATUS_KIND_WALK{
        }
        acmd!(lua_state, {
            SLOW_OPPONENT(1.1, 600)
           //SLOW_OPPONENT(0.1, 1000)
        });
       // println!("{}", prev_status);
        let previous_statuses = [PREV_STATUS_1[ENTRY_ID],PREV_STATUS_2[ENTRY_ID] , PREV_STATUS_3[ENTRY_ID], PREV_STATUS_4[ENTRY_ID], PREV_STATUS_5[ENTRY_ID], PREV_STATUS_6[ENTRY_ID], PREV_STATUS_7[ENTRY_ID], PREV_STATUS_8[ENTRY_ID], PREV_STATUS_9[ENTRY_ID], PREV_STATUS_10[ENTRY_ID]];
        //println!("{}", PREV_STATUS_1[ENTRY_ID]);
        let mut att:i32 = 0;
        if ENTRY_ID > 8 {
            ENTRY_ID = 8;
        }
        let mut opponent_pos: f32 = -1.0;
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE\
      9instance_E\u{0}"
                .as_bytes()
                .as_ptr(), );
        let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        let special: [i32; 225] = [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_L, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_EAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, *ITEM_PACMANKEY_STATUS_KIND_SPECIAL_HAVE, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_BLOW, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_PASS, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_WAIT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_FIRE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BOMB, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_FOOD, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_ITEM, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP];
        let is_aerial:bool = false;

        tech_everything(module_accessor);
        auto_turnaround(module_accessor);
        //walk_stuff(module_accessor);
        //combo_break(get_module_accessor(0));
        //combo_break(get_module_accessor(1));
        throw_cancels(module_accessor);
        ad_cancels(module_accessor);
        //off_the_top_sd(module_accessor);
        no_lag_shield(module_accessor);
        disable_turn(module_accessor);
        if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_JUMP,
        *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind){
            disable_catch(module_accessor);
            disable_jab(module_accessor);
        }
        dash_attack(module_accessor);
       // StatusModule::change_status_request_from_script(module_accessor, STATUS_KIND[ENTRY_ID], true);
        if !is_damage_check(module_accessor) && ![*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind){
            enable_jump(module_accessor);
            enable_dash_force(module_accessor);
        }
             
        if !smash::app::sv_information::is_ready_go(){
            //PostureModule::set_pos_2d(module_accessor, &smash::phx::Vector2f{x: 0.0, y: 0.0});
            //StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            //enable_jump(module_accessor);
        }
        if status_kind == *FIGHTER_STATUS_KIND_TURN{
            //StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        }

        if status_kind == *FIGHTER_STATUS_KIND_SQUAT && MotionModule::frame(module_accessor) < 5.0{
            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
        //air_taunt(module_accessor);
        //whiff_stuff(module_accessor);
        if [hash40("appeal_s_l"), hash40("appeal_s_r"), hash40("appeal_hi_l"), hash40("appeal_hi_r"), hash40("appeal_lw_l"), hash40("appeal_lw_r")].contains(&MotionModule::motion_kind(module_accessor)){
            if MotionModule::frame(module_accessor) <= 1.0{
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            }
            else{
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            }
        }

        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){
            if ![*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_JUMP,
            *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind){
                CancelModule::enable_cancel(module_accessor);
            if [*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_100].contains(&status_kind){
                //enable_smash_atk_force(module_accessor);
                enable_tilts_force(module_accessor);
            }
            enable = true;
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH{
                //enable_tilts_force(module_accessor);
                //enable_jab_force(module_accessor);
                enable_tilts(module_accessor);
                enable_jab(module_accessor);
            }
            else{
                disable_jab(module_accessor);
                disable_tilts(module_accessor);
            }
            if [*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind){
                enable_smash_atk_force(module_accessor);
            }
            //enable_smash_atk_force(module_accessor);
                enable_jump_force(module_accessor);
            if !special.contains(&status_kind){
                enable_specials_force(module_accessor);
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_100{
                enable_tilts_force(module_accessor);
            }
            else{
                disable_jab(module_accessor);
                disable_smash_atks(module_accessor);
            }
            //WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
            enable_dash_force(module_accessor);
            if !special.contains(&status_kind){
                enable_aerials(module_accessor);
            }
            else{
                disable_aerials(module_accessor);
            }
        
        }
        }
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD){
            if [*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, 
            *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind){
                enable_dash_force(module_accessor);
                enable_jump_force(module_accessor);
            }
        }
        if MotionModule::motion_kind(module_accessor) == smash::hash40("squat"){
            //MotionModule::change_motion(module_accessor, smash::phx::Hash40{hash: hash40("escape")}, 0.0, 1.0, false, 0.0, false, false);
        }

        if [*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind){
            //enable_jump(module_accessor);
            //enable_smash_atk_force(module_accessor);
            CancelModule::enable_cancel(module_accessor);
            if MotionModule::frame(module_accessor) < 35.0{
                disable_jump(module_accessor);
                disable_smash_atks(module_accessor);
                disable_tilts(module_accessor);
                disable_aerials(module_accessor);
                disable_jab(module_accessor);
                //disable_specials(module_accessor);
                disable_walk(module_accessor);
                disable_dash(module_accessor);
                disable_run(module_accessor);
                disable_turn(module_accessor);
                disable_crouch(module_accessor);
                disable_jab_100(module_accessor);
            }
            else{
                enable_all(module_accessor);
            }
            
        }
        if [*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_100].contains(&status_kind) {
            //enable_jump(module_accessor);
            CancelModule::enable_cancel(module_accessor);
            if MotionModule::frame(module_accessor) < 35.0{
                disable_jump(module_accessor);
            disable_smash_atks(module_accessor);
            //disable_tilts(module_accessor);
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_100{
                disable_jab_100(module_accessor);
            }
            disable_aerials(module_accessor);
            disable_specials(module_accessor);
            disable_walk(module_accessor);
            disable_dash(module_accessor);
            disable_run(module_accessor);
            disable_jab(module_accessor);
            disable_turn(module_accessor);
            disable_crouch(module_accessor);
            disable_catch(module_accessor);
            //disable_jab_100(module_accessor);
            }
            else{
                enable_all(module_accessor);
            }
            
        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
            //FighterManager::set_position_lock(FIGHTER_MANAGER, smash::app::FighterEntryID(ENTRY_ID as i32), true);
        }
        else{
            //FighterManager::set_position_lock(FIGHTER_MANAGER, smash::app::FighterEntryID(ENTRY_ID as i32), false);
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind){
            //enable_jump(module_accessor);
            CancelModule::enable_cancel(module_accessor);
            disable_jump(module_accessor);
            disable_smash_atks(module_accessor);
            disable_tilts(module_accessor);
            disable_shield(module_accessor);
            disable_aerials(module_accessor);
            disable_jab(module_accessor);
            disable_specials(module_accessor);
            disable_walk(module_accessor);
            disable_dash(module_accessor);
            disable_run(module_accessor);
            disable_turn(module_accessor);
            disable_ground_dodge(module_accessor);
            disable_crouch(module_accessor);
            disable_jab_100(module_accessor);
        }
        if status_kind == *FIGHTER_STATUS_KIND_RUN{
            CancelModule::enable_cancel(module_accessor);
        }
        if special.contains(&status_kind){
            enable_jump(module_accessor);
            CancelModule::enable_cancel(module_accessor);
            //disable_smash_atks(module_accessor);
            //disable_tilts(module_accessor);
            if MotionModule::frame(module_accessor) < 35.0{
                disable_aerials(module_accessor);
                //disable_jab(module_accessor);
                disable_specials(module_accessor);
                //disable_walk(module_accessor);
                disable_dash(module_accessor);
                disable_run(module_accessor);
                disable_turn(module_accessor);
            }
            else{
                enable_aerials(module_accessor);
                enable_specials(module_accessor);
            }
           
        }
        else{
            //CAN_CANCEL = false;
        }
    
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR{
            CancelModule::enable_cancel(module_accessor);
            //enable = false;
            disable_aerials(module_accessor);
            disable_jump(module_accessor);
            disable_smash_atks(module_accessor);
            disable_ground_dodge(module_accessor);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
             

        }
        else{
            enable = false;
        }
        if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF{
            CancelModule::enable_cancel(module_accessor);
        }
        if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END{
            //StatusModule::set_situation_kind(module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
            if MotionModule::frame(module_accessor) == MotionModule::end_frame(module_accessor){
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            }
            }
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&status_kind)  {
            MotionModule::set_rate(module_accessor, 2.0);
            enable_jump_force(module_accessor);
            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL{
            CancelModule::enable_cancel(module_accessor);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind) && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
            //WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD, true);
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
        if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI{
            FighterKineticEnergyGravity::set_gravity_coefficient(fighter_kinetic_energy_motion, 1.3)
        } else {
            FighterKineticEnergyGravity::set_gravity_coefficient(fighter_kinetic_energy_motion, 0.9)
        }
        if ControlModule::get_stick_y(module_accessor) > 0.5 && y_vel <= 0.0{
            FighterKineticEnergyGravity::set_gravity_coefficient(fighter_kinetic_energy_motion, 0.5)
        }
        /*
        if special.contains(&status_kind){
            enable_jump(module_accessor);
            CancelModule::enable_cancel(module_accessor);
            //disable_smash_atks(module_accessor);
            //disable_tilts(module_accessor);
            //disable_aerials(module_accessor);
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
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
        }

        if special.contains(&status_kind) || MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi_air") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi"){
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
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
    smashline::install_status_scripts!(fall_status_main);
}
