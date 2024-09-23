use super::*;

unsafe extern "C" fn game_specialairhi3end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, DEADFALL);
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    whiff_cancel(fighter);
}    

pub fn install() {
    Agent::new("miigunner")
        .game_acmd("game_specialairhi3end", game_specialairhi3end, Priority::Low)
        .install();
}
