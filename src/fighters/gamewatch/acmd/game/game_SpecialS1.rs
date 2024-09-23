use super::*;

unsafe extern "C" fn game_specials1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(fighter.module_accessor, Hash40::new("panel"));
        
    }
}    

pub fn install() {
    Agent::new("gamewatch")
        .game_acmd("game_specials1", game_specials1, Priority::Low)
        .install();
}
