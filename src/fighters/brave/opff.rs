
use super::*;

unsafe extern "C" fn  brave_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        
        
        
        
        
        
         

    }                                      
}

pub fn install() {
    Agent::new("brave")
    .on_line(Main, brave_opff)
    .install();
}