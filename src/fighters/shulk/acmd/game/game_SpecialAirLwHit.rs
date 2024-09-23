use super::*;



unsafe extern "C" fn game_specialairlwhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        enable_counter_cancel(fighter);
    }

}    

pub fn install() {
    Agent::new("shulk")
        .game_acmd("game_specialairlwhit", game_specialairlwhit, Priority::Low)
        .install();
}
