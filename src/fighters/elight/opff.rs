use super::*;
//use crate::fighters::elight::effect::effect_speciallw;
use crate::fighters::element::status::special_lw::{SWAP, swap_aegis};



//#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
unsafe extern "C" fn elight_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        
        //sWAP ON ATTACK CANCEL
        if ENABLE_ATTACK_CANCEL[entry_id]
        && (AttackModule::is_attack_occur(fighter.module_accessor) 
        && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) == 0) {
            if (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0) 
            && SWAP[entry_id] == false {
                SWAP[entry_id] = true;
                //effect_speciallw(fighter);
                //ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
        }

        if SWAP[entry_id] {
            swap_aegis(fighter);
        }
    }                                      
}

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(elight_opff);

}