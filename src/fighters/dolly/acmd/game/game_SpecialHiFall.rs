use super::*;

unsafe extern "C" fn game_specialhifall(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        //whiff_cancel(fighter);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
}    

pub fn install() {
    Agent::new("dolly")
        .game_acmd("game_specialhifall", game_specialhifall, Priority::Low)
        .install();
}
