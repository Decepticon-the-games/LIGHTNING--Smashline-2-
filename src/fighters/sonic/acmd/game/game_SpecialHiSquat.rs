use super::*;

unsafe extern "C" fn game_specialhisquat(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP) {
        if macros::is_excute(fighter) {
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}    

pub fn install() {
    Agent::new("sonic")
        .game_acmd("game_specialhisquat", game_specialhisquat, Priority::Low)
        .install();
}
