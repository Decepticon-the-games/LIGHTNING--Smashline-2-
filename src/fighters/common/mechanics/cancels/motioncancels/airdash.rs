use super::*;

unsafe extern "C" fn airdash(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);  


    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {

            //Airdash
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("escape_air_slide") && frame >= 8.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }  
        }
    }
}

    

pub fn install() {
    Agent::new("fighter")
    .on_line(Main,airdash)
    .install()
} 