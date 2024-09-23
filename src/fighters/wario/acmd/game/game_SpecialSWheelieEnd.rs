use super::*;

unsafe extern "C" fn game_specialswheelieend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    macros::FT_MOTION_RATE(fighter, 0.6);
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_S_FLAG_WHEELIE_END);
    }
}    

pub fn install() {
    Agent::new("wario")
        .game_acmd("game_specialswheelieend", game_specialswheelieend, Priority::Low)
        .install();
}
