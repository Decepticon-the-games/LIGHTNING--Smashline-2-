use super::*;

unsafe extern "C" fn game_specialairs1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FLAMEPILLAR, false, -1);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("miigunner")
        .game_acmd("game_specialairs1", game_specialairs1, Priority::Low)
        .install();
}