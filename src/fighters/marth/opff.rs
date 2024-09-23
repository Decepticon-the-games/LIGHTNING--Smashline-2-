use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_MARTH )]

unsafe extern "C" fn marth_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
                
            
            
            
            


        }
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(marth_opff);

}