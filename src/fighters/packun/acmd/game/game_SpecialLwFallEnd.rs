use super::*;

unsafe extern "C" fn game_speciallwfallend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    wait(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_END_CHANGE_KINETIC);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("packun")
        .game_acmd("game_speciallwfallend", game_speciallwfallend, Priority::Low)
        .install();
}
