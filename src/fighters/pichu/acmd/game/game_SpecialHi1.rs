use super::*;

unsafe extern "C" fn game_specialhi1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 0.5);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

pub fn install() {
    Agent::new("pichu")
        .game_acmd("game_specialhi1", game_specialhi1, Priority::Low)
        .install();
}
