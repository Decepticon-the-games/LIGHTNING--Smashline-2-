use super::*;
use crate::fighters::common::mechanics::{cancels::motioncancels::cancel_in_neutral::CANCEL_IN_NEUTRAL, lightning_mechanics::vanish::WEAPON_BOMA};

pub const ATTACK_CANCEL: i32 = 0x106;
pub const ENABLE_ATTACK_CANCEL: i32 = 0x107;
pub const ENABLE_MULTIHIT_CANCEL: i32 = 0x0108;
pub const MULTIHIT_CANCEL: i32 = 0x0109;
pub const MULTIHIT_COUNT : i32 = 0x010A;
//These attack_cancel functions get the state your in (status_kind, flag, or motion_kind), your next button, the previous state you were in.

//ATTACK_CANCELS (attacks cancellable on the hitbox wherever this function is placed, typically the one that launches an opponent)
pub unsafe extern "C" fn enable_attack_cancel(fighter : &mut L2CAgentBase) {
    let oboma = sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32); // links weapon to whatever may ownn it  
    let battle_object_category = utility::get_category(&mut *fighter.module_accessor);
    
    //Check for if the hitbox belongs to a fighter or weapon, then enable attack cancelling for the fighter.
    if ! AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        if battle_object_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            WorkModule::on_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL);
        }
        if battle_object_category == *BATTLE_OBJECT_CATEGORY_WEAPON {
           WorkModule::on_flag(oboma, ENABLE_ATTACK_CANCEL); 
        }
    }
}
pub unsafe extern "C" fn attack_cancel(fighter : &mut L2CFighterCommon) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;   
        let activate_founder_id = (WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID));
        let link_owner = (WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER));
        let oboma = sv_battle_object::module_accessor(activate_founder_id as  u32); // links weapon to whatever may ownn it  
        let weapon_boma = sv_battle_object::module_accessor(WEAPON_BOMA[entry_id]); //Accounts for non conventional weapons, such as Bayo wickedweave and Kamui dragon
        
    if entry_id < 1 {
        //println!("en_attack_cancel: {}", WorkModule::is_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL));
        //println!("en_attack_cancel: {}", WorkModule::is_flag(oboma, ENABLE_ATTACK_CANCEL));
        //println!("attack_cancel: {}", WorkModule::is_flag(fighter.module_accessor, ATTACK_CANCEL));
        //println!("inflict_oboma: {}", AttackModule::is_infliction(weapon_boma, *COLLISION_KIND_MASK_ALL));
    } 

    if entry_id < 8 {
        //Check sonce enabling is on for a hit, and then allows for cancelling
        if WorkModule::is_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL) {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
            || AttackModule::is_infliction(weapon_boma, *COLLISION_KIND_MASK_ALL) { 
                WorkModule::on_flag(fighter.module_accessor, ATTACK_CANCEL);
                WorkModule::off_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL);  
            }
        }
        //Turn off once CANCEL_IN_NEUTRAL runs/Hitboxes are cleared
        else if WorkModule::is_flag(fighter.module_accessor, CANCEL_IN_NEUTRAL) { 
            WorkModule::off_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL);
        }       
        
        //Attack Cancel
        if WorkModule::is_flag(fighter.module_accessor, ATTACK_CANCEL) {
            cancel_on_hit(fighter);   
            
            //Extend buffer frames each frame of hitlag
            ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 

            //Turn on visible window in trainining mode
            /*if app::smashball::is_training_mode() {
               macros::FLASH(fighter, 0, 0.55, 1, 1.75);  
            }*/
        }

        //reset every motion/status change
        if StatusModule::is_changing(fighter.module_accessor)
        || MotionModule::is_changing(fighter.module_accessor) {
            WorkModule::off_flag(fighter.module_accessor, ATTACK_CANCEL);
            WorkModule::off_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL);
        }
    }
}

//MULTIHIT_CANCELS (multihit attacks)
pub unsafe extern "C" fn enable_multihit_cancel(fighter : &mut L2CAgentBase) {
    let oboma = sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32); // links weapon to whatever may ownn it  
    let battle_object_category = utility::get_category(&mut *fighter.module_accessor);
    
    //Check for if the hitbox belongs to a fighter or weapon, then enable attack cancelling for the fighter.
    if ! AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        if battle_object_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            WorkModule::on_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL);
        }
        if battle_object_category == *BATTLE_OBJECT_CATEGORY_WEAPON {
           WorkModule::on_flag(oboma, ENABLE_MULTIHIT_CANCEL); 
        }
    }
}
//(canceling at a specific point in a multihit move)
pub unsafe extern "C" fn multihit_cancel(
    fighter: &mut L2CAgentBase, 
    status: i32, 
    flag: i32,
    motion: u64,
    next_input: bool, 
    status_reset: i32,
    flag_reset: i32, 
    motion_reset: u64) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    let is_condition = (
    StatusModule::status_kind(fighter.module_accessor) == status
    || WorkModule::is_flag(fighter.module_accessor, flag)
    || MotionModule::motion_kind(fighter.module_accessor) == motion
    );

    let is_reset = (
    StatusModule::status_kind(fighter.module_accessor) != status_reset
    || !WorkModule::is_flag(fighter.module_accessor, flag_reset)
    || MotionModule::motion_kind(fighter.module_accessor) != motion_reset
    );

    if WorkModule::is_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL) {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
            WorkModule::on_flag(fighter.module_accessor, MULTIHIT_CANCEL);   
            WorkModule::off_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL);                 
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, MULTIHIT_CANCEL) {
        if is_condition {
            if next_input {
                cancel_on_hit(fighter); 
                //WorkModule::off_flag(fighter.module_accessor, MULTIHIT_CANCEL);                 
            }   
            //Extend buffer frames each frame of hitlag
            ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
        }
        else if is_reset {
            WorkModule::off_flag(fighter.module_accessor, MULTIHIT_CANCEL);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL)
    || WorkModule::is_flag(fighter.module_accessor, ATTACK_CANCEL) {
        WorkModule::off_flag(fighter.module_accessor, MULTIHIT_CANCEL);
        WorkModule::off_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL);
    }
    

    if entry_id < 1 {
        //println!("en_mul_cancel: {}", WorkModule::is_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL));
        //println!("mul_cancel: {}", WorkModule::is_flag(fighter.module_accessor, MULTIHIT_CANCEL));
    } 
}
//(canceling after landing a ceertain amount of hits in a multihit move)
pub unsafe extern "C" fn multihit_counter(
    fighter: &mut L2CAgentBase,
    status: i32,
    flag: i32,
    motion: u64,
    multihitcount: i32,
    next_input: bool,
    status_reset: i32,
    flag_reset: i32,
    motion_reset: u64) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    let is_condition = (
    StatusModule::status_kind(fighter.module_accessor) == status
    || WorkModule::is_flag(fighter.module_accessor, flag)
    || MotionModule::motion_kind(fighter.module_accessor) == motion
    );

    let is_reset = (
    StatusModule::status_kind(fighter.module_accessor) != status_reset
    || !WorkModule::is_flag(fighter.module_accessor, flag_reset)
    || MotionModule::motion_kind(fighter.module_accessor) != motion
    );
    
    

    if WorkModule::is_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL) {

        //Multihit cancels after a certain amount of successful hits
        if is_condition {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                WorkModule::inc_int(fighter.module_accessor, MULTIHIT_COUNT)         
            }
        
            if WorkModule::get_int(fighter.module_accessor, MULTIHIT_COUNT) == multihitcount { //how many hits
                WorkModule::on_flag(fighter.module_accessor, MULTIHIT_CANCEL);
                WorkModule::off_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL);                 
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, MULTIHIT_CANCEL) {
        if is_condition {
                if next_input {
                cancel_on_hit(fighter);   
            }
            //Extend buffer frames each frame of hitlag
            ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 

        }
        else if is_reset {
            WorkModule::set_int(fighter.module_accessor, 0, MULTIHIT_COUNT);
        }  
    }  
    if WorkModule::is_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL)
    || WorkModule::is_flag(fighter.module_accessor, ATTACK_CANCEL) {
        WorkModule::off_flag(fighter.module_accessor, MULTIHIT_CANCEL);
        WorkModule::off_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL);
    }

    if entry_id < 1 {
        //println!("en_mul_cancel: {}", WorkModule::is_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL));
        //println!("mul_count: {}", WorkModule::get_int(fighter.module_accessor, MULTIHIT_COUNT));
    } 
}

//Custom if after hitlag function, only true if is_attack_occur aside from hitlag.
pub unsafe extern "C" fn is_after_hitlag(fighter: &mut L2CAgentBase) -> bool {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    ! AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && ! (SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 || StopModule::is_stop(fighter.module_accessor))
}
unsafe extern "C" fn is_after_hitlag_weapon(weapon: &mut L2CAgentBase) -> bool {
    let entry_id = WorkModule::get_int(weapon.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    ! AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ALL)
    && ! (SlowModule::frame(weapon.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 || StopModule::is_stop(weapon.module_accessor))
}

//If an attack occurs, cancel out of attack (after hitlag)
unsafe extern "C" fn cancel_on_hit(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    
    //if WorkModule::is_flag(fighter.module_accessor, ATTACK_CANCEL) {
        if is_after_hitlag(fighter) {
            if (moveset_input(fighter) || whiff_input(fighter)) {
                CancelModule::enable_cancel(fighter.module_accessor);    
            } 
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                WorkModule::off_flag(fighter.module_accessor, ATTACK_CANCEL);
            }         
        }
    //}
}
unsafe extern "C" fn cancel_on_hit_resets(fighter : &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, ENABLE_ATTACK_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, ATTACK_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, ENABLE_MULTIHIT_CANCEL);
    WorkModule::set_int(fighter.module_accessor, 0, MULTIHIT_COUNT);
}
pub fn install() {
    Agent::new("fighter")
    .on_line(Main, attack_cancel)
    .on_start(cancel_on_hit_resets)
    .install();
}