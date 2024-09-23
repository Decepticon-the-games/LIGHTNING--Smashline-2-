use super::*;

unsafe extern "C" fn game_specialsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        whiff_cancel(fighter);
    }
}   

pub fn install() {
    Agent::new("ryu")
        .game_acmd("game_specialsend", game_specialsend, Priority::Low)
        .install();
}
