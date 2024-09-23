use super::*;

unsafe extern "C" fn game_specialhilandingf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
whiff_cancel(fighter);
    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
}    

pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_specialhilandingf", game_specialhilandingf, Priority::Low)
        .install();
}
