use super::*;

unsafe extern "C" fn game_specialairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_BINDBALL, false, -1);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("mewtwo")
        .game_acmd("game_specialairlw", game_specialairlw, Priority::Low)
        .install();
}
