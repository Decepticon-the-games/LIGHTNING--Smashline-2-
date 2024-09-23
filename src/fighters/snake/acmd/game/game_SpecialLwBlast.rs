use super::*;

unsafe extern "C" fn game_speciallwblast(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH);
    }
    frame(fighter.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
    }
    frame(fighter.lua_state_agent, 29.0);
}pub fn install() {
    Agent::new("snake")
        .game_acmd("game_speciallwblast", game_speciallwblast, Priority::Low)
        .install();
}
