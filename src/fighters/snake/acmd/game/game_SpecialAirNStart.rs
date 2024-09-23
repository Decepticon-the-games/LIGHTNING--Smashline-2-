use super::*;

unsafe extern "C" fn game_specialairnstart(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
}pub fn install() {
    Agent::new("snake")
        .game_acmd("game_specialairnstart", game_specialairnstart, Priority::Low)
        .install();
}
