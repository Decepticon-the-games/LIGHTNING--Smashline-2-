use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_SHULK )]

unsafe extern "C" fn shulk_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
                
            let frame = MotionModule::frame(fighter.module_accessor);
            
            
            


//FIX UP B CANCEL 
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && frame >20.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }

        }
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(shulk_opff);

}