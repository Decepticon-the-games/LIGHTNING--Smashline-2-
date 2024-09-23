use super::*;

unsafe extern "C" fn game_specialairs1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_BOOMERANG, false, -1);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_BOOMERANG, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);        
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("younglink")
        .game_acmd("game_specialairs1", game_specialairs1, Priority::Low)
        .install();
}
