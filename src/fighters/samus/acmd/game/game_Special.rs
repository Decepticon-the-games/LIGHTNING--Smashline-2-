use super::*;

unsafe extern "C" fn game_special(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("samus")
        .game_acmd("game_special", game_special, Priority::Low)
        .install();
}
