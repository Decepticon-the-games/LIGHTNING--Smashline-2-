use super::*;

unsafe extern "C" fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("littlemac")
        .game_acmd("game_speciallw", game_speciallw, Priority::Low)
        .install();
}