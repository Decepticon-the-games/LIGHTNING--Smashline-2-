use super::*;

unsafe extern "C" fn game_specialnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
whiff_cancel(fighter);
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.65);
}    

pub fn install() {
    Agent::new("inkling")
        .game_acmd("game_specialnend", game_specialnend, Priority::Low)
        .install();
}
