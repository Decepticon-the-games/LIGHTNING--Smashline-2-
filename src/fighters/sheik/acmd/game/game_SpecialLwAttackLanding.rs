use super::*;

unsafe extern "C" fn game_speciallwattacklanding(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("sheik")
        .game_acmd("game_speciallwattacklanding", game_speciallwattacklanding, Priority::Low)
        .install();
}
