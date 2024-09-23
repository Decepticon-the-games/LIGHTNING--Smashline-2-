use super::*;

unsafe extern "C" fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.2);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CLAY);
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 1.06);
}    

pub fn install() {
    Agent::new("duckhunt")
        .game_acmd("game_specials", game_specials, Priority::Low)
        .install();
}
