use super::*;

unsafe extern "C" fn game_specialairsjumpend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
whiff_cancel(fighter);
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 1.0, 9.0);
        macros::HIT_NODE(fighter, Hash40::new("trans"), *HIT_STATUS_OFF);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_S_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5.0, 5.0);
    }
}    

pub fn install() {
    Agent::new("inkling")
        .game_acmd("game_specialairsjumpend", game_specialairsjumpend, Priority::Low)
        .install();
}
