use super::*;

unsafe extern "C" fn game_specialairnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_FLAG_SHOOT);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_specialairnshoot", game_specialairnshoot, Priority::Low)
        .install();
}
