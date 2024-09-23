use super::*;

unsafe extern "C" fn game_specialairlw(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("link")
        .game_acmd("game_specialairlw", game_specialairlw, Priority::Low)
        .install();
}
