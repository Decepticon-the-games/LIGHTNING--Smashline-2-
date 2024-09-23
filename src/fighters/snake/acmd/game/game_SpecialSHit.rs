use super::*;

unsafe extern "C" fn game_specialshit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }  
}  

pub fn install() {
    Agent::new("snake")
        .game_acmd("game_specialshit", game_specialshit, Priority::Low)
        .install();
}
