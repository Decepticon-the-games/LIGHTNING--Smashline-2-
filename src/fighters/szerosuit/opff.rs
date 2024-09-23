use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_SZEROSUIT )]

unsafe extern "C" fn szerosuit_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
                
            let frame = MotionModule::frame(fighter.module_accessor);
            
            let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
            




            //In Lightning...
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) {
                //Cancel up special into down special before the last hit if sucesfully hit  
                let next_input = (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0)
                multihit_cancel(fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI, 0, 0, next_input, *FIGHTER_STATUS_KIND_SPECIAL_HI, 0, 0);
            }
        }
    }

pub fn install() {
    Agent::new("szerosuit")
    .on_line(Main, szerosuit_opff)
    .install();

}