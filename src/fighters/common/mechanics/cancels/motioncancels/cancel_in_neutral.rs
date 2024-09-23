use super::*;
use crate::fighters::common::mechanics::cancels::attack_cancels::cancel_on_hit::ENABLE_ATTACK_CANCEL;
use crate::fighters::common::mechanics::lightning_mechanics::lightning_mode::FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING;

pub const CANCEL_IN_NEUTRAL: i32 = 0x0100;
pub const DEADFALL: i32 = 0x0101;//HANDLES ALL DEADFALL MOVES.
pub const WHIFF_CANCEL: i32 = 0x0103;//Effects only


//Whiff Cancel into dash, jump, dodges, and shield.

pub unsafe extern "C" fn whiff_cancel(fighter : &mut L2CAgentBase) {//This function goes out to ACMD moveset  scrips so they can read this code.
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let battle_object_category = utility::get_category(&mut *fighter.module_accessor);
    let oboma = sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32); // links weapon to whatever may ownn it  

    if ! (AttackModule::is_attack_occur(fighter.module_accessor)
    || AttackModule::is_attack_occur(oboma)) {
        if battle_object_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            if ! WorkModule::is_flag(fighter.module_accessor, ATTACK_CANCEL) {
                WorkModule::on_flag(fighter.module_accessor, CANCEL_IN_NEUTRAL);
                WorkModule::off_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL);
            }
        }
        else if battle_object_category == *BATTLE_OBJECT_CATEGORY_WEAPON {
            if ! WorkModule::is_flag(oboma, ATTACK_CANCEL) {
                WorkModule::on_flag(oboma, CANCEL_IN_NEUTRAL);
                WorkModule::off_flag(oboma, ENABLE_ATTACK_CANCEL);
            }
        }
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, CANCEL_IN_NEUTRAL);
    }
}
unsafe extern "C" fn deadfall_whiff_cancel(fighter : &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if WorkModule::get_int(fighter.module_accessor, DEADFALL) != 0 {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR 
        || StopModule::is_hit(fighter.module_accessor)
        || CaptureModule::is_capture(fighter.module_accessor) {

            WorkModule::set_int(fighter.module_accessor, 0, DEADFALL);
        }        
    }
}

pub unsafe extern "C" fn cancel_in_neutral(fighter : &mut L2CFighterCommon) {   
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;     
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as u8;

        if entry_id < 1 {
            //println!("cin: {}", WorkModule::is_flag(fighter.module_accessor, CANCEL_IN_NEUTRAL));
            //println!("m_changing: {}", MotionModule::is_changing(fighter.module_accessor));
            //println!("s_changing: {}", StatusModule::is_changing(fighter.module_accessor));
            //println!("efx: {}", WorkModule::is_flag(fighter.module_accessor, WHIFF_CANCEL));
        } 
        if entry_id < 8 {

            //Cancel in neutral
            if WorkModule::is_flag(fighter.module_accessor, CANCEL_IN_NEUTRAL) { 
                WorkModule::on_flag(fighter.module_accessor, WHIFF_CANCEL);
                if [
                    *FIGHTER_STATUS_KIND_DASH,
                    *FIGHTER_STATUS_KIND_TURN_DASH,
                    *FIGHTER_STATUS_KIND_JUMP,
                    *FIGHTER_STATUS_KIND_JUMP_SQUAT,
                    *FIGHTER_STATUS_KIND_JUMP_AERIAL,         
                    *FIGHTER_STATUS_KIND_GUARD,
                    *FIGHTER_STATUS_KIND_GUARD_ON,
                    *FIGHTER_STATUS_KIND_ESCAPE, 
                    *FIGHTER_STATUS_KIND_ESCAPE_F, 
                    *FIGHTER_STATUS_KIND_ESCAPE_B,
                    *FIGHTER_STATUS_KIND_ESCAPE_AIR, 
                    *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE           
                ].contains(&status_kind) 
                || StopModule::is_hit(fighter.module_accessor)
                || MotionModule::frame(fighter.module_accessor) == FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) 
                || StatusModule::is_changing(fighter.module_accessor)
                || MotionModule::is_changing(fighter.module_accessor)
                {
                    WorkModule::off_flag(fighter.module_accessor, CANCEL_IN_NEUTRAL);
                } 
                else {                    
                    ControlModule::set_command_life_extend(fighter.module_accessor, (cancel_frame - frame as u8)); //sets life extend to however many frames r left before you hit the normal cancel frame, so u always have accurate buffer.
                    if whiff_input(fighter) { 
                        CancelModule::enable_cancel(fighter.module_accessor); //When in cancel in neutral, u can only cancel into dash, jump, dodges, and shield. See transition_terms.rs
                    }
                }
            } 
        }  
    }
}
//EFFECTS
pub unsafe extern "C" fn whiff_cancel_effects(fighter : &mut L2CFighterCommon) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
    if WorkModule::is_flag(fighter.module_accessor, WHIFF_CANCEL) {
        if [
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_TURN_DASH,
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_SQUAT,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,         
            *FIGHTER_STATUS_KIND_GUARD,
            *FIGHTER_STATUS_KIND_GUARD_ON,
            *FIGHTER_STATUS_KIND_ESCAPE, 
            *FIGHTER_STATUS_KIND_ESCAPE_F, 
            *FIGHTER_STATUS_KIND_ESCAPE_B,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR, 
            *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE           
        ].contains(&status_kind) 
        {
            macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.5);
        }
        else {
           WorkModule::off_flag(fighter.module_accessor, WHIFF_CANCEL);
        }
    }
}
unsafe extern "C" fn resets(fighter : &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, CANCEL_IN_NEUTRAL);
    WorkModule::set_int(fighter.module_accessor, 0, DEADFALL);
    WorkModule::off_flag(fighter.module_accessor, WHIFF_CANCEL);
}
pub fn install() {
    Agent::new("fighter")
    .on_line(Main, cancel_in_neutral)
    .on_line(Main, whiff_cancel_effects)
    .on_line(Main, deadfall_whiff_cancel)
    .on_start(resets)
    .install();
}