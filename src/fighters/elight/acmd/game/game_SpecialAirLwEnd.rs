use super::*;

unsafe extern "C" fn game_specialairlwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
CancelModule::enable_cancel(fighter.module_accessor);

}

pub fn install() {
    Agent::new("elight")
        .game_acmd("game_specialairlwend", game_specialairlwend, Priority::Low)
        .install();
}
