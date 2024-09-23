use super::*;

unsafe extern "C" fn game_specialairsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_2);
    }
}

pub fn install() {
    Agent::new("zelda")
        .game_acmd("game_specialairsend", game_specialairsend, Priority::Low)
        .install();
}
