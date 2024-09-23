use super::*;

unsafe extern "C" fn game_speciallwattack(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    Agent::new("zelda")
        .game_acmd("game_speciallwattack", game_speciallwattack, Priority::Low)
        .install();
}
