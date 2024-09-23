use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
unsafe extern "C" fn edge_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        ////let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        
    }                                      
}

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(edge_opff);

}