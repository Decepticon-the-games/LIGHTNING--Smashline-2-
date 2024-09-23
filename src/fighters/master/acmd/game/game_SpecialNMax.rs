use super::*;

unsafe extern "C" fn game_specialnmax(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1) {
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            whiff_cancel(fighter);
        }
    }
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW2, false, -1);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW2, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        macros::SET_SPEED_EX(fighter, -1, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}    

pub fn install() {
    Agent::new("master")
        .game_acmd("game_specialnmax", game_specialnmax, Priority::Low)
        .install();
}
