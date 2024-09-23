use super::*;

unsafe extern "C" fn game_specialsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
        whiff_cancel(fighter);
    }
}  

pub fn install() {
    Agent::new("ike")
        .game_acmd("game_specialsend", game_specialsend, Priority::Low)
        .install();
}
