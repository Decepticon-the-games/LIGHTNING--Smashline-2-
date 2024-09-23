use super::*;

unsafe extern "C" fn game_specialairhiendf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
whiff_cancel(fighter);


    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}    

pub fn install() {
    Agent::new("jack")
        .game_acmd("game_specialairhiendf", game_specialairhiendf, Priority::Low)
        .install();
}
