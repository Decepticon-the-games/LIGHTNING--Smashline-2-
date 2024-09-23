use super::*;
pub static mut FASTFALL_LASER : [bool; 8] = [false; 8];

//#[fighter_frame( agent = FIGHTER_KIND_WOLF )]

unsafe extern "C" fn wolf_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            
            
            

//Fast fall laser
            if motion_kind == smash::hash40("special_air_n") && FASTFALL_LASER[entry_id] {
                if (ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
                    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    FASTFALL_LASER[entry_id] = false;
                }                
            }
            else {
                FASTFALL_LASER[entry_id] = false;
            }
//illusion
            //if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            //    if AttackModule::is_attack_occur(fighter.module_accessor) {
                    //if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                    //|| (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0
                    //|| (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
                        //CancelModule::enable_cancel(fighter.module_accessor);
                    //}
            //    }   
            //}
        }
    }


pub fn install() {
    Agent::new("wolf")
    .on_line(Main, wolf_opff)
    .install();

}