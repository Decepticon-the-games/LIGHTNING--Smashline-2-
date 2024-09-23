use super::*;

unsafe extern "C" fn game_specialairlwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
whiff_cancel(fighter);
    wait(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_END_CHANGE_KINETIC);
    }
}    

pub fn install() {
    Agent::new("packun")
        .game_acmd("game_specialairlwend", game_specialairlwend, Priority::Low)
        .install();
}
