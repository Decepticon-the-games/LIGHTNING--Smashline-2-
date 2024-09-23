use super::*;

unsafe extern "C" fn game_specialairlwn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
}pub fn install() {
    Agent::new("shulk")
        .game_acmd("game_specialairlwn", game_specialairlwn, Priority::Low)
        .install();
}
