use super::*;
unsafe extern "C" fn game_specialairlw1end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

whiff_cancel(fighter);
}    
pub fn install() {
    Agent::new("miigunner")
        .game_acmd("game_specialairlw1end", game_specialairlw1end, Priority::Low)
        .install();
}