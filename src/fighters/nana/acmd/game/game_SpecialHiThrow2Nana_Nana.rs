use super::*;

unsafe extern "C" fn game_specialhithrow2nana_nana(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, DEADFALL);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_CLIFF_PULL_PARTNER_FLAG_PULL);
    }
}

pub fn install() {
    Agent::new("nana")
        .game_acmd("game_specialhithrow2nana_nana", game_specialhithrow2nana_nana, Priority::Low)
        .install();
}
