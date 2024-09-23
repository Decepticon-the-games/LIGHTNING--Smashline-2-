use super::*;

unsafe extern "C" fn game_specialairlwstart(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_START_WAIT_INPUT);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_KICK_ENABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_REVERSE);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_WALL_JUMP_ENABLE);
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
    }
}pub fn install() {
    Agent::new("szerosuit")
        .game_acmd("game_specialairlwstart", game_specialairlwstart, Priority::Low)
        .install();
}
