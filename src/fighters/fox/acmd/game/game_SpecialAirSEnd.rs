use super::*;

unsafe extern "C" fn game_specialairsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_AIR_CONTROL);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
}    

pub fn install() {
    Agent::new("fox")
        .game_acmd("game_specialairsend", game_specialairsend, Priority::Low)
        .install();
}
