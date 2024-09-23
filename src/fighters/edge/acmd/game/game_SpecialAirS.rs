use super::*;

unsafe extern "C" fn game_specialairs(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_EDGE_GENERATE_ARTICLE_FLARE1, false, -1);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("edge")
        .game_acmd("game_specialairs", game_specialairs, Priority::Low)
        .install();
}
