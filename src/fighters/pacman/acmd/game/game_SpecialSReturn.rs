use super::*;

unsafe extern "C" fn game_specialsreturn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    whiff_cancel(fighter);
}    

pub fn install() {
    Agent::new("pacman")
        .game_acmd("game_specialsreturn", game_specialsreturn, Priority::Low)
        .install();
}
