use super::*;

unsafe extern "C" fn game_speciallwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    whiff_cancel(fighter);
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_LOOP);   
    }
}    

pub fn install() {
    Agent::new("gamewatch")
        .game_acmd("game_speciallwend", game_speciallwend, Priority::Low)
        .install();
}
