use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_KROOL )]

unsafe extern "C" fn krool_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            
                
            let frame = MotionModule::frame(fighter.module_accessor);
            
            
            

        }
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(krool_opff);

}