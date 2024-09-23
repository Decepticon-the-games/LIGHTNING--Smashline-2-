use super::*;

unsafe extern "C" fn game_speciallwkicklanding(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 1.2);
}pub fn install() {
    Agent::new("szerosuit")
        .game_acmd("game_speciallwkicklanding", game_speciallwkicklanding, Priority::Low)
        .install();
}
