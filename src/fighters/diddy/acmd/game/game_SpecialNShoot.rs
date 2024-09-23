use super::*;

unsafe extern "C" fn game_specialnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_PEANUTS, false, -1);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("diddy")
        .game_acmd("game_specialnshoot", game_specialnshoot, Priority::Low)
        .install();
}
