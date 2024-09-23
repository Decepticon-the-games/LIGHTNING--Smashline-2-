use super::*;

unsafe extern "C" fn mario_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let frame = MotionModule::frame(fighter.module_accessor);
        
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        
            
        //In Lightning...
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) {
            //Cancel Dair only right before last hit
            let next_input = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0;
            multihit_cancel(fighter, 0, 0, smash::hash40("attack_air_lw"), next_input, 0, 0, smash::hash40("attack_air_lw")); 
        }
    }
}

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_opff)
    .install();
}