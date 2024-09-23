use super::*;

unsafe extern "C" fn game_specialairsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
whiff_cancel(fighter);
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5.0, 5.0);
        macros::HIT_NODE(fighter, Hash40::new("trans"), *HIT_STATUS_OFF);
    }
}    

pub fn install() {
    Agent::new("inkling")
        .game_acmd("game_specialairsend", game_specialairsend, Priority::Low)
        .install();
}
