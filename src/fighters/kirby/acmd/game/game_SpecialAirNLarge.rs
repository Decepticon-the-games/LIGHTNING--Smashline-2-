use super::*;

unsafe extern "C" fn game_specialairnlarge(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;


    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_REMOVE);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_ITEM_USE);
    }
}

pub fn install() {
    Agent::new("kirby")
        .game_acmd("game_specialairnlarge", game_specialairnlarge, Priority::Low)
        .install();
}
