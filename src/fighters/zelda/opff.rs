use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_ZELDA )]

unsafe extern "C" fn zelda_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            
        }
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(zelda_opff);

}