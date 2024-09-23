use super::*;

unsafe extern "C" fn game_specialairnfire3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_GENERATE_BULLET);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_START_PRECEDE_CHECK);
    }
}    

pub fn install() {
    Agent::new("buddy")
        .game_acmd("game_specialairnfire3", game_specialairnfire3, Priority::Low)
        .install();
}
