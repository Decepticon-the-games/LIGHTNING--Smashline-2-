use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_CLOUD )]

unsafe extern "C" fn cloud_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
                
            let frame = MotionModule::frame(fighter.module_accessor);
            
            
            

            //In Lightning...
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) {
                //Up B cancel is free if attack occurs  
                let next_input = ! 
                multihit_cancel(fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI, 0, 0, next_input, *FIGHTER_STATUS_KIND_SPECIAL_HI, 0, 0);
            }

        }
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(cloud_opff);

}