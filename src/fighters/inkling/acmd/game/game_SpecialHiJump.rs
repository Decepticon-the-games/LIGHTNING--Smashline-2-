use super::*;

unsafe extern "C" fn game_specialhijump(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASH, false, -1);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
    
}    

pub fn install() {
    Agent::new("inkling")
        .game_acmd("game_specialhijump", game_specialhijump, Priority::Low)
        .install();
}
