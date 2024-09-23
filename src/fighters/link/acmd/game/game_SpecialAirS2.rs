use super::*;

unsafe extern "C" fn game_specialairs2(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("link")
        .game_acmd("game_specialairs2", game_specialairs2, Priority::Low)
        .install();
}
