use super::*;

unsafe extern "C" fn game_specialairlw7(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 5.0);
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STRENGTH);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_DEATHBALL, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 53.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}    

pub fn install() {
    Agent::new("brave")
        .game_acmd("game_specialairlw7", game_specialairlw7, Priority::Low)
        .install();
}
