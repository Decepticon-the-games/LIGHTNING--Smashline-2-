use super::*;

unsafe extern "C" fn game_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("younglink")
        .game_acmd("game_specialn", game_specialn, Priority::Low)
        .install();
}