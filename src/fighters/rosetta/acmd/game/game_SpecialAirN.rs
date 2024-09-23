use super::*;

unsafe extern "C" fn game_specialairn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROSETTA_STATUS_SPECIAL_N_FLAG_CHARGE);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROSETTA_STATUS_SPECIAL_N_FLAG_SHOOT);
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 0.833);
    frame(fighter.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(fighter, 0.9);
}    

pub fn install() {
    Agent::new("rosetta")
        .game_acmd("game_specialairn", game_specialairn, Priority::Low)
        .install();
}
