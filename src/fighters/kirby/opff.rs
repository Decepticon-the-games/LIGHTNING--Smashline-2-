use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]

unsafe extern "C" fn kirby_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            





        


//New subtititle for any other code, if not applicable just delete the lines

        }
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(kirby_opff);

}