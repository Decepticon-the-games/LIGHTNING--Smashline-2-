use super::*;

unsafe extern "C" fn game_specialnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    whiff_cancel(fighter);
}    

pub fn install() {
    Agent::new("toonlink")
        .game_acmd("game_specialnend", game_specialnend, Priority::Low)
        .install();
}
