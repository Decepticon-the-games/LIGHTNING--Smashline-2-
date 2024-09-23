use super::*;

unsafe extern "C" fn game_specialnrebound(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.15);
}pub fn install() {
    Agent::new("sonic")
        .game_acmd("game_specialnrebound", game_specialnrebound, Priority::Low)
        .install();
}
