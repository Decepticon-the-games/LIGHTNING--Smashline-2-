use super::*;

unsafe extern "C" fn game_specials32(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("miigunner")
        .game_acmd("game_specials32", game_specials32, Priority::Low)
        .install();
}