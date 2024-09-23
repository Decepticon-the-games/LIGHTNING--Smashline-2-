use super::*;

unsafe extern "C" fn game_specialairnhold(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_CHARGE);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("samusd")
        .game_acmd("game_specialairnhold", game_specialairnhold, Priority::Low)
        .install();
}
