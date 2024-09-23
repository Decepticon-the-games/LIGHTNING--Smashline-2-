use super::*;

unsafe extern "C" fn game_specialairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SHULK_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}    

pub fn install() {
    Agent::new("shulk")
        .game_acmd("game_specialairlw", game_specialairlw, Priority::Low)
        .install();
}
