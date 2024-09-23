use super::*;

unsafe extern "C" fn game_specialnspin(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 50);
    }
}    

pub fn install() {
    Agent::new("sonic")
        .game_acmd("game_specialnspin", game_specialnspin, Priority::Low)
        .install();
}
