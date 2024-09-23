use super::*;

unsafe extern "C" fn game_specialairnturn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("peach")
        .game_acmd("game_specialairnturn", game_specialairnturn, Priority::Low)
        .install();
}
