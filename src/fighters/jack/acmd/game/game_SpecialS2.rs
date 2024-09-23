use super::*;

unsafe extern "C" fn game_specials2(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *WEAPON_JACK_DOYLE_GENERATE_ARTICLE_FIRE2, false, -1);
        if entry_id < 8 {
            whiff_cancel(fighter); 
        }
    }
}    

pub fn install() {
    Agent::new("jack_doyle")
        .game_acmd("game_specials2", game_specials2, Priority::Low)
        .install();
}
