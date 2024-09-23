use super::*;

unsafe extern "C" fn game_specialnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_TRY);
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}    

pub fn install() {
    Agent::new("reflet")
        .game_acmd("game_specialnshoot", game_specialnshoot, Priority::Low)
        .install();
}
