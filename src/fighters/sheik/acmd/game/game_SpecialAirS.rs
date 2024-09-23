use super::*;

unsafe extern "C" fn game_specialairs(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_THROW);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("sheik")
        .game_acmd("game_specialairs", game_specialairs, Priority::Low)
        .install();
}
