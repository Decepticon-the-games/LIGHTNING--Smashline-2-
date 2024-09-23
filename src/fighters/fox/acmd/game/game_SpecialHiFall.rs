use super::*;

unsafe extern "C" fn game_specialhifall(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
        //whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("fox")
        .game_acmd("game_specialhifall", game_specialhifall, Priority::Low)
        .install();
}
