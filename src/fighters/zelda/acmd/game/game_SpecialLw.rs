use super::*;

unsafe extern "C" fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(fighter, 0.9);
}

pub fn install() {
    Agent::new("zelda")
        .game_acmd("game_speciallw", game_speciallw, Priority::Low)
        .install();
}
