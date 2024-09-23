use super::*;

unsafe extern "C" fn game_specialairnstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
}    

pub fn install() {
    Agent::new("donkey")
        .game_acmd("game_specialairnstart", game_specialairnstart, Priority::Low)
        .install();
}
