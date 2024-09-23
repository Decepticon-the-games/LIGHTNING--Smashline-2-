use super::*;

unsafe extern "C" fn game_specialairs(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.88);
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_FALL);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("murabito")
        .game_acmd("game_specialairs", game_specialairs, Priority::Low)
        .install();
}
