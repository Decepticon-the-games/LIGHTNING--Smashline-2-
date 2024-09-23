use super::*;

unsafe extern "C" fn game_specialairhithrow_nana(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, DEADFALL);
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_FLAG_FAIL_CLIFF_START);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("popo")
        .game_acmd("game_specialairhithrow_nana", game_specialairhithrow_nana, Priority::Low)
        .install();
}
