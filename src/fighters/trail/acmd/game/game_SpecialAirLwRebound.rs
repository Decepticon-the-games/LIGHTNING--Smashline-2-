use super::*;

unsafe extern "C" fn game_specialairlwrebound(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        enable_counter_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}    

pub fn install() {
    Agent::new("trail")
        .game_acmd("game_specialairlwrebound", game_specialairlwrebound, Priority::Low)
        .install();
}
