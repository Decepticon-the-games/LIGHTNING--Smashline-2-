use super::*;

unsafe extern "C" fn game_specialairs2end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
whiff_cancel(fighter);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_FLAG_2);   
    }
}    

pub fn install() {
    Agent::new("miigunner")
        .game_acmd("game_specialairs2end", game_specialairs2end, Priority::Low)
        .install();
}
