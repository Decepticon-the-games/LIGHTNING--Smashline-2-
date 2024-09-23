use super::*;
unsafe extern "C" fn game_speciallw1end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

whiff_cancel(fighter);
}    
pub fn install() {
    Agent::new("miigunner")
        .game_acmd("game_speciallw1end", game_speciallw1end, Priority::Low)
        .install();
}