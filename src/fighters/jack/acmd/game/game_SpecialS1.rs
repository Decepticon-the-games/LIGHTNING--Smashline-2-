use super::*;

unsafe extern "C" fn game_specials1(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_FIRE, false, -1);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("jack")
        .game_acmd("game_specials1", game_specials1, Priority::Low)
        .install();
}
