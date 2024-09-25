use super::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40; 
use skyline::hooks::{getRegionAddress, Region};
use smash::app::FighterManager;


use crate::fighters::common::mechanics::cancels::whiff_input;

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(module_accessor);
    
    let moveset_terms = (term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2 
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
        
    let other_terms = (term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);

    if WorkModule::get_int(module_accessor, DEADFALL) != 0 {
        if term == WorkModule::get_int(module_accessor, DEADFALL) {
            return false;  
        }
        else {
            return ret;
        }
    }
    else if WorkModule::is_flag(module_accessor, CANCEL_IN_NEUTRAL) {
        if moveset_terms || other_terms {
           return false; 
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

pub fn install() {

    skyline::install_hooks!(
        is_enable_transition_term_replace
    );

}