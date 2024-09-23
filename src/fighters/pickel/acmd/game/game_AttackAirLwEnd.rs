use super::*;

unsafe extern "C" fn game_attackairlwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}pub fn install() {
    Agent::new("pickel")
        .game_acmd("game_attackairlwend", game_attackairlwend, Priority::Low)
        .install();
}
