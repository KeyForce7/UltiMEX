use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterBase,L2CFighterCommon};
use acmd::{acmd, acmd_func};
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::BattleObjectModuleAccessor;
use std::mem;
use smash::app::sv_module_access;
use smash::app::sv_battle_object;
use smash::phx::Vector4f;
use skyline::nn::ro::LookupSymbol;
static mut FIGHTER_MANAGER_ADDR: usize = 0;
static mut AIRDODGE : [i32; 9] = [2; 9];
static mut POSX : [f32; 9] = [-1.0 ; 9];
static mut OPPONENT_ID : [usize; 9] = [9; 9];
static mut AIRTAUNT_USED :[bool; 9] =[false;9];
static mut TECH_FRAME: [i32; 9] = [0; 9];
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


pub fn get_module_accessor(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
    unsafe {
        &mut *smash::app::sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(entry_id))
    }
}

pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if ENTRY_ID>8{
            ENTRY_ID = 8;
        }
        let mut opponent_pos: f32 = -1.0;
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),);
        let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);

        let special: [i32 ; 225] = [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_L, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_EAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, *ITEM_PACMANKEY_STATUS_KIND_SPECIAL_HAVE, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET,  *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_BLOW, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_PASS, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_WAIT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_FIRE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BOMB, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_FOOD, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_ITEM, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END,  *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING,*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP];


        //jab
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && [*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_100].contains(&status_kind) {
            CancelModule::enable_cancel(module_accessor);
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
            if ControlModule::get_stick_y(module_accessor)>0.3 && ControlModule::get_stick_x(module_accessor)<0.5 && ControlModule::get_stick_x(module_accessor) > -0.5{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            }
        }


        //dash attack jab cancellable anywhere and movement cancellable on hit
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL){
            CancelModule::enable_cancel(module_accessor);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
            WorkModule::enable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
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
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH && !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL){
            if  ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_ATTACK){
                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_ATTACK,true);
            }
            if ControlModule::get_stick_y(module_accessor)>0.5 && ControlModule::get_stick_x(module_accessor)<0.5 && ControlModule::get_stick_x(module_accessor) > -0.5{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            }
            if ControlModule::get_stick_y(module_accessor)<0.5 && ControlModule::get_stick_x(module_accessor)<0.5 && ControlModule::get_stick_x(module_accessor) > -0.5{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);
            }
        }






        //tilts
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && [*FIGHTER_STATUS_KIND_ATTACK_S3,*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
            let cancelframe_t = FighterMotionModuleImpl::get_cancel_frame(module_accessor, smash::phx::Hash40::new_raw(MotionModule::motion_kind(module_accessor)), true) as f32;
            CancelModule::enable_cancel(module_accessor);
            if MotionModule::frame(module_accessor) <= cancelframe_t{
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
                WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
            }

            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);


            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
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


            if MotionModule::frame(module_accessor) >= cancelframe_t{
                WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
                WorkModule::enable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
                WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
            }
            /*if ControlModule::get_stick_y(module_accessor)>0.3 && ControlModule::get_stick_x(module_accessor)<0.5 && ControlModule::get_stick_x(module_accessor) > -0.5{
              StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            }*/
        }
        //smash
        if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){
            CancelModule::enable_cancel(module_accessor);
            WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
            WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
            WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
            WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
            WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
            WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD){ WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);}

            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);

        }
        //fox trot down flick
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DASH{
            if ControlModule::get_stick_y(module_accessor) == -1.0{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
            }
        }
        //smash attack hold cancellable
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind) && (ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD)){
            WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_GUARD,true);
        }
        //aerials
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && !special.contains(&status_kind) && StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR {
            let cancelframe_s = FighterMotionModuleImpl::get_cancel_frame(module_accessor, smash::phx::Hash40::new_raw(MotionModule::motion_kind(module_accessor)), true) as f32;
            CancelModule::enable_cancel(module_accessor);
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


            if MotionModule::frame(module_accessor) <= cancelframe_s  {
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
            }
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
            WorkModule::unable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
            //WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
            if MotionModule::frame(module_accessor) >= cancelframe_s || StatusModule::prev_status_kind(module_accessor,0)!= *FIGHTER_STATUS_KIND_ATTACK_AIR {
                WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
                WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
                WorkModule::enable_transition_term_group_ex_all(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
                WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
            }
        }






        //special
        if special.contains(&status_kind) || MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi_air") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi") || AttackModule::is_attack(module_accessor, 0, false) && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            CancelModule::enable_cancel(module_accessor);
            //disable for now might revert
            //WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL){
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
            }
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
            //for lucario up b
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
        }





        //attack to movement on shield or whiff apparently
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) || (AttackModule::is_attack(module_accessor, 0, false) && !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
            if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind){
                CancelModule::enable_cancel(module_accessor);
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
                WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);


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
                //63
                if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
                    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
                    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
                    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
                    WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
                }
                if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind){
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
                    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);

                }
                if [*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind){
                    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
                }
            }
        }











        //throw cancels
        if status_kind == *FIGHTER_STATUS_KIND_THROW {
            //new: throw is special cancellable****************************************************************************************
            CancelModule::enable_cancel(module_accessor);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
            //WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
            //WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
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
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);

            //***************************************************************************************************************
            WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
            //WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
            WorkModule::unable_transition_term_group_ex(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
        }



        //side special out of a run
        if status_kind == *FIGHTER_STATUS_KIND_RUN {
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
        }



        //airdash

        if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_GROUND || StatusModule::situation_kind(module_accessor) == SITUATION_KIND_CLIFF{
            AIRDODGE[ENTRY_ID] = 2;
        }
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR{
            AIRDODGE[ENTRY_ID] += 2;
        }
        if status_kind == *FIGHTER_STATUS_KIND_TREAD_JUMP{
            AIRDODGE[ENTRY_ID]+=1;
        }
        if AIRDODGE[ENTRY_ID] >= 4 {
            AIRDODGE[ENTRY_ID] = 4;
        }




        if AIRDODGE[ENTRY_ID] > 0 {
            WorkModule::set_float(module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_AIR);
            WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        }
        if AIRDODGE[ENTRY_ID] <= 0{
            AIRDODGE[ENTRY_ID] = 0;
        }
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind){
            AIRDODGE[ENTRY_ID] -= 1;
            WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        }

        if CancelModule::is_enable_cancel(module_accessor) && StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR && ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_GUARD) && AIRDODGE[ENTRY_ID]>0{
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }





        //up b fix
        if special.contains(&status_kind){
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP){

                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);

            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
                if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR && AIRDODGE[ENTRY_ID] > 0{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
            if MotionModule::frame(module_accessor)==MotionModule::end_frame(module_accessor){
                WorkModule::enable_transition_term_group_ex(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
                WorkModule::unable_transition_term_group_ex(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
            }

        }

        //taunt
        if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR{
            let stop_rise  = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 1.0 };
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI){
                MotionModule::change_motion(module_accessor, smash::phx::Hash40{hash: hash40("appeal_hi_l")}, 0.0, 1.0, false, 0.0, false, false);
                CancelModule::enable_cancel(module_accessor);
                if AIRTAUNT_USED[ENTRY_ID] == false{
                    KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    KineticModule::mul_accel(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    AIRTAUNT_USED[ENTRY_ID] = true;
                }
            }else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW){
                MotionModule::change_motion(module_accessor, smash::phx::Hash40{hash: hash40("appeal_lw_l")}, 0.0, 1.0, false, 0.0, false, false);
                CancelModule::enable_cancel(module_accessor);
                if AIRTAUNT_USED[ENTRY_ID] == false{
                    KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    KineticModule::mul_accel(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    AIRTAUNT_USED[ENTRY_ID] = true;
                }
            }else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L){
                MotionModule::change_motion(module_accessor, smash::phx::Hash40{hash: hash40("appeal_s_l")}, 0.0, 1.0, false, 0.0, false, false);
                CancelModule::enable_cancel(module_accessor);
                if AIRTAUNT_USED[ENTRY_ID] == false{
                    KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    KineticModule::mul_accel(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    AIRTAUNT_USED[ENTRY_ID] = true;
                }
            }else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R){
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
        //autoturnaround
        if FighterManager::entry_count(FIGHTER_MANAGER)==2 && !smash::app::smashball::is_training_mode(){
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

            if fighter_kind != *FIGHTER_KIND_KEN && fighter_kind != *FIGHTER_KIND_RYU && fighter_kind != *FIGHTER_KIND_DOLLY{
                if POSX[ENTRY_ID] < opponent_pos && PostureModule::lr(module_accessor) == -1.0 && [*FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) /*to the left and facing left*/{

                    StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_TURN,true);

                }else if POSX[ENTRY_ID] > opponent_pos && PostureModule::lr(module_accessor) == 1.0 && [*FIGHTER_STATUS_KIND_WAIT].contains(&status_kind){

                    StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_TURN,true);

                }
            }

        }
//no more free fall
        if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL{
            StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_FALL, true);
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
        if status_kind == *FIGHTER_STATUS_KIND_DASH {
            MotionModule::set_rate(module_accessor, 2.0);
        }
//tech everything
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR{
            TECH_FRAME[ENTRY_ID]+=1;
        } else {
            TECH_FRAME[ENTRY_ID] = 0;
        }
        if TECH_FRAME[ENTRY_ID]<=20 && TECH_FRAME[ENTRY_ID]>0{
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASSIVE_WALL, true);
            }
        }
//flash yellow when crouch cancelling
        /*if [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_SQUAT_WAIT].contains(&status_kind) && StatusModule::prev_status_kind(module_accessor,0)!=*FIGHTER_STATUS_KIND_WAIT{
          let mut colorflashvec1 = Vector4f { /* Red */ x : 0.0, /* Green */ y : 0.0, /* Blue */ z : 0.0, /* Alpha? */ w : 0.1}; // setting this and the next vector's .w to 1 seems to cause a ghostly effect
          let mut colorflashvec2 = Vector4f { /* Red */ x : 1.0, /* Green */ y : 1.0, /* Blue */ z : 0.0, /* Alpha? */ w : 0.1};
          ColorBlendModule::set_main_color(module_accessor, &colorflashvec1, &colorflashvec2, 0.7, 0.2, 75, true);
        }else{

          let mut colorflashvec2 = Vector4f { /* Red */ x : 0.0, /* Green */ y : 0.0, /* Blue */ z : 0.0, /* Alpha? */ w : 0.0};
          ColorBlendModule::set_main_color(module_accessor, &colorflashvec2, &colorflashvec2, 0.7, 0.2, 75, true);
        }
        */
//dash tilt stick cancels

    }


}










pub fn install(){
    //skyline::install_hook!(is_generatable_hook);
    acmd::add_custom_hooks!(once_per_fighter_frame);

}