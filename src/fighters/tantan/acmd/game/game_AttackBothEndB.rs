use super::*;

unsafe extern "C" fn game_attackbothendb(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("tantan")
        .game_acmd("game_attackbothendb", game_attackbothendb, Priority::Low)
        .install();
}