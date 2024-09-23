use super::*;

unsafe extern "C" fn game_specialsendb(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    macros::FT_MOTION_RATE(fighter, 0.66);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
}    

pub fn install() {
    Agent::new("gekkouga")
        .game_acmd("game_specialsendb", game_specialsendb, Priority::Low)
        .install();
}
