use super::*;

unsafe extern "C" fn game_specialsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 28.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    whiff_cancel(fighter);
}    

pub fn install() {
    Agent::new("luigi")
        .game_acmd("game_specialsend", game_specialsend, Priority::Low)
        .install();
}
