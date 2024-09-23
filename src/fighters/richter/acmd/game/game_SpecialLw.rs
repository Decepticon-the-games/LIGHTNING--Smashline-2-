use super::*;

unsafe extern "C" fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("richter")
        .game_acmd("game_speciallw", game_speciallw, Priority::Low)
        .install();
}
