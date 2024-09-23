use super::*;

unsafe extern "C" fn game_specialairsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("buddy")
        .game_acmd("game_specialairsend", game_specialairsend, Priority::Low)
        .install();
}
