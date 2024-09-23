use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_KEN )]

unsafe extern "C" fn ken_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor,  *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            
                
            let frame = MotionModule::frame(fighter.module_accessor);
            
            
            


                
//New subtititle for any other code, if not applicable just delete the lines

        }
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(ken_opff);

}