use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_PACMAN )]

unsafe extern "C" fn pacman_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            
            
            


        }
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(pacman_opff);

}