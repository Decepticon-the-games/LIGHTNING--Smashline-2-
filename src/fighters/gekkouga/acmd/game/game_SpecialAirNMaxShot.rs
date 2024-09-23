use super::*;

unsafe extern "C" fn game_specialairnmaxshot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_N_WORK_FLAG_SHURIKEN_SHOOT);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("gekkouga")
        .game_acmd("game_specialairnmaxshot", game_specialairnmaxshot, Priority::Low)
        .install();
}
