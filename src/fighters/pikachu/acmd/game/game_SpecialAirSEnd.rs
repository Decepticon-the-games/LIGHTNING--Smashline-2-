use super::*;

unsafe extern "C" fn game_specialairsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        JostleModule::set_status(fighter.module_accessor, true);
        whiff_cancel(fighter);
    }
} 

pub fn install() {
    Agent::new("pikachu")
        .game_acmd("game_specialairsend", game_specialairsend, Priority::Low)
        .install();
}