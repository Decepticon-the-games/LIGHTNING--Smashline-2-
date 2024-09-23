use super::*;

unsafe extern "C" fn game_specialhiend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

whiff_cancel(fighter); 
}    

pub fn install() {
    Agent::new("robot")
        .game_acmd("game_specialhiend", game_specialhiend, Priority::Low)
        .install();
}
