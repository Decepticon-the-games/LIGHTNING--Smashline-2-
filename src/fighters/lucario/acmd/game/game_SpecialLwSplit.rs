use super::*;


unsafe extern "C" fn game_speciallwsplit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        enable_counter_cancel(fighter);
    }
    if macros::is_excute(fighter) {
        
        macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}    

pub fn install() {
    Agent::new("lucario")
        .game_acmd("game_speciallwsplit", game_speciallwsplit, Priority::Low)
        .install();
}
