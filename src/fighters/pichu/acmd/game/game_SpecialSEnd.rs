use super::*;

unsafe extern "C" fn game_specialsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("pichu")
        .game_acmd("game_specialsend", game_specialsend, Priority::Low)
        .install();
}
