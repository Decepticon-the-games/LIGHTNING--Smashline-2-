use super::*;

unsafe extern "C" fn game_specialairlwshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_LW_SHOOT_WORK_ID_FLAG_REQUEST_SHOOT);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("rockman")
        .game_acmd("game_specialairlwshoot", game_specialairlwshoot, Priority::Low)
        .install();
}
