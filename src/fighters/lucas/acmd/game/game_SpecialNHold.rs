use super::*;

unsafe extern "C" fn game_specialnhold(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 0.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
}    

pub fn install() {
    Agent::new("lucas")
        .game_acmd("game_specialnhold", game_specialnhold, Priority::Low)
        .install();
}
