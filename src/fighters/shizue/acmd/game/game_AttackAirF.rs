use super::*;

unsafe extern "C" fn game_attackairf(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("shizue_slingshot")
        .game_acmd("game_attackairf", game_attackairf, Priority::Low)
        .install();
}
