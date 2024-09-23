use super::*;

unsafe extern "C" fn game_specialsstart(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_1);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("zelda")
        .game_acmd("game_specialsstart", game_specialsstart, Priority::Low)
        .install();
}
