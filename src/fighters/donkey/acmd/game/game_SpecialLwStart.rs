use super::*;

unsafe extern "C" fn game_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
}pub fn install() {
    Agent::new("donkey")
        .game_acmd("game_speciallwstart", game_speciallwstart, Priority::Low)
        .install();
}
