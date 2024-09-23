use super::*;

unsafe extern "C" fn game_specialairnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
whiff_cancel(fighter);
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.65);
}    

pub fn install() {
    Agent::new("inkling")
        .game_acmd("game_specialairnend", game_specialairnend, Priority::Low)
        .install();
}
