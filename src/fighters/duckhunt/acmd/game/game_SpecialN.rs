use super::*;

unsafe extern "C" fn game_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CAN);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("duckhunt")
        .game_acmd("game_specialn", game_specialn, Priority::Low)
        .install();
}