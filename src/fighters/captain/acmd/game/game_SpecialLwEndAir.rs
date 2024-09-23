use super::*;

unsafe extern "C" fn game_speciallwendair(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_FALL_ONOFF);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_CLIFF_CHECK);
                whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("captain")
        .game_acmd("game_speciallwendair", game_speciallwendair, Priority::Low)
        .install();
}
