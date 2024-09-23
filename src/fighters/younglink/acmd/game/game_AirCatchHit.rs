use super::*;

unsafe extern "C" fn game_aircatchhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
}    

pub fn install() {
    Agent::new("younglink")
        .game_acmd("game_aircatchhit", game_aircatchhit, Priority::Low)
        .install();
}
