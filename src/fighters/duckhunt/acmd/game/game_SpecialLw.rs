use super::*;

unsafe extern "C" fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DUCKHUNT_STATUS_SPECIAL_LW_FLAG_CALL_TRIGGER);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("duckhunt")
        .game_acmd("game_speciallw", game_speciallw, Priority::Low)
        .install();
}