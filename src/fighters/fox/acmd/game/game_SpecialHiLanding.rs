use super::*;

unsafe extern "C" fn game_specialhilanding(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
}


pub fn install() {
    Agent::new("fox")
        .game_acmd("game_specialhilanding", game_specialhilanding, Priority::Low)
        .install();
}
